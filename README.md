# Social API

API de rede social gerada com o [Gize](https://github.com/robertolima-dev/gize) 0.9.10 e atualizada para a 0.9.12 (veja [Atualizar o Gize](#atualizar-o-gize)). O stack é Axum, SQLx e PostgreSQL. O projeto tem autenticação JWT, especificação OpenAPI, endpoint WebSocket e um painel admin em React.

Os recursos saíram dos geradores do Gize. Por cima deles foram escritas à mão as regras que uma rede social precisa e que o CRUD gerado não tem: cada linha tem dono, mensagens são privadas e likes e follows não se repetem. A seção [Personalizações](#personalizações) lista o que mudou.

## Modelo de dados

```
users ──< posts ──< comments
  │          └────< likes
  ├──< profiles (1 por usuário)
  ├──< follows (follower → followee, as duas pontas em users)
  ├──< selfies
  └──< messages (sender → recipient, as duas pontas em users)
hashtags
```

| Recurso | Campos | Relações | Dono |
| --- | --- | --- | --- |
| `users` | name, email, password, is_admin | vem pronto com o `gize new` | |
| `profiles` | bio, avatar_url | `user` → users | `user_id` |
| `posts` | body, published | `author` → users | `author_id` |
| `comments` | body | `post` → posts, `author` → users | `author_id` |
| `likes` | | `post` → posts, `user` → users | `user_id` |
| `follows` | | `follower` → users, `followee` → users | `follower_id` |
| `selfies` | image_url, caption | `author` → users | `author_id` |
| `messages` | body, read | `sender` → users, `recipient` → users | `sender_id` |
| `hashtags` | name | | |

## Pré-requisitos

- Rust 1.85 ou mais novo
- PostgreSQL
- Node.js, só para rodar o admin

## Passo a passo

### 1. Instalar o Gize

```sh
cargo install gize --version 0.9.12 --locked
gize --version   # gize 0.9.12
```

### 2. Criar o projeto e os recursos

```sh
gize new social --openapi --ws
cd social

gize make crud Profile bio:String avatar_url:String user:belongs_to:users
gize make crud Post    body:String published:bool author:belongs_to:users
gize make crud Comment body:String post:belongs_to:posts author:belongs_to:users
gize make crud Like    post:belongs_to:posts user:belongs_to:users
gize make crud Follow  follower:belongs_to:users followee:belongs_to:users
gize make crud Selfie  image_url:String caption:String author:belongs_to:users
gize make crud Message body:String read:bool sender:belongs_to:users recipient:belongs_to:users
gize make crud Hashtag name:String
gize make admin
```

`Selfie` vira a tabela `selfies`. Como o nome não pode ser reconstruído a partir da tabela (daria `Selfy`), o Gize grava `model = "Selfie"` no `gize.toml`, e o `gize sync` mantém o nome certo.

### 3. Configurar o banco e aplicar as migrations

```sh
cp .env.example .env    # ajuste DATABASE_URL, PORT e GIZE_JWT_SECRET
createdb gize_social
gize migrate
gize createadmin        # pede email, nome e senha
```

### 4. Rodar

```sh
gize serve              # API na PORT do .env e o admin
gize serve --api-only   # só a API
```

## Personalizações

Tudo abaixo é código seu. O Gize marca esses arquivos como `drift` no `gize sync` e não os toca sem `--force`.

- **`src/shared/mod.rs`** (novo, declarado em `src/main.rs`): `caller` lê o id do usuário no token, e `ensure_owner` deixa passar o dono da linha ou um admin.
- **`handler.rs` de cada recurso com dono:**
  - No `create`, o dono vem do token. O valor enviado pelo cliente é ignorado.
  - No `update` e no `delete`, só o dono ou um admin passa; os outros recebem `403`. O dono de uma linha nunca muda.
- **`dto.rs`:** o campo de dono ganhou `#[serde(default)]`, então o cliente pode omiti-lo.
- **`error.rs`:** variante `Forbidden`, que responde `403`.
- **`messages`** (`routes.rs`, `handler.rs`, `service.rs`, `repository.rs`):
  - Todas as rotas exigem token.
  - A listagem traz só as mensagens que o usuário enviou ou recebeu; um admin vê todas.
  - A mensagem de outra pessoa responde `404`.
  - O destinatário só pode mudar `read`; o texto e as pontas ficam como estavam.
- **`follows/handler.rs`:** seguir a si mesmo responde `422`.
- **`migrations/…_social_rules.sql`** (escrita à mão, criada com `gize make migration social_rules`):
  - Um like por usuário e post, um follow por par e um perfil por usuário. A repetição responde `409`.
  - Apagar um post apaga também os comentários e likes dele.

`hashtags` ficou como o Gize gerou: qualquer usuário logado cria tags.

## Regras de acesso

| Rota | Acesso |
| --- | --- |
| `POST /users/register`, `POST /users/login` | público |
| `GET /users/me` | qualquer token válido |
| `/users` e `/users/:id` | só admin |
| `GET` em posts, comments, likes, follows, profiles, selfies, hashtags | público |
| `POST` nesses recursos | qualquer token válido; o dono é quem chama |
| `PUT` e `DELETE` nesses recursos | o dono ou um admin |
| `/messages` (todas as rotas) | remetente, destinatário ou admin |

## Testar a API

```sh
API=localhost:8080   # use a PORT do seu .env
J='content-type: application/json'

ANA=$(curl -s $API/users/register -H "$J" \
  -d '{"name":"Ana","email":"ana@example.com","password":"longenough1"}' | jq -r .token)
BRUNO=$(curl -s $API/users/register -H "$J" \
  -d '{"name":"Bruno","email":"bruno@example.com","password":"longenough1"}' | jq -r .token)
ANA_ID=$(curl -s $API/users/me -H "authorization: Bearer $ANA" | jq -r .id)
BRUNO_ID=$(curl -s $API/users/me -H "authorization: Bearer $BRUNO" | jq -r .id)

# Post da Ana (o autor vem do token)
POST=$(curl -s $API/posts -H "$J" -H "authorization: Bearer $ANA" \
  -d '{"body":"Primeiro post","published":true}' | jq -r .id)

# Bruno comenta, curte e segue a Ana
curl -s $API/comments -H "$J" -H "authorization: Bearer $BRUNO" -d "{\"body\":\"Lindo!\",\"post_id\":\"$POST\"}"
curl -s $API/likes    -H "$J" -H "authorization: Bearer $BRUNO" -d "{\"post_id\":\"$POST\"}"
curl -s $API/follows  -H "$J" -H "authorization: Bearer $BRUNO" -d "{\"followee_id\":\"$ANA_ID\"}"

# Bruno não edita o post da Ana: 403
curl -s -o /dev/null -w '%{http_code}\n' -X PUT $API/posts/$POST -H "$J" \
  -H "authorization: Bearer $BRUNO" -d '{"body":"x","published":true}'

# Mensagem privada
curl -s $API/messages -H "$J" -H "authorization: Bearer $ANA" \
  -d "{\"body\":\"Oi!\",\"read\":false,\"recipient_id\":\"$BRUNO_ID\"}"
curl -s $API/messages -H "authorization: Bearer $BRUNO"   # Bruno vê a mensagem
curl -s -o /dev/null -w '%{http_code}\n' $API/messages     # sem token: 401

# Listagem paginada: {items, page, per_page, total}
curl -s "$API/posts?page=1&per_page=25"
```

### Outras rotas

- `GET /openapi.json` devolve a especificação OpenAPI. Ela também está salva em `openapi.json` na raiz do projeto.
- `GET /docs` abre a documentação interativa.
- `GET /ws` é o endpoint WebSocket.

## Atualizar o Gize

O `gize sync` nunca sobrescreve um arquivo que difere do que o Gize gera, então numa atualização
ele lista juntos os arquivos editados à mão e os que mudaram no template. O `--force` sobrescreveria
os dois. O caminho seguro é trazer só os arquivos que o CHANGELOG da versão nomeia.

Foi assim na passagem da 0.9.10 para a 0.9.12:

```sh
cargo install gize --version 0.9.12 --locked
gize sync --dry-run    # 30 em drift: as 24 personalizações + 6 arquivos do template

# Gera a versão nova numa cópia e traz só os 6 arquivos citados no CHANGELOG
cp -R . /tmp/social-regen && (cd /tmp/social-regen && gize sync --force)
for f in src/app/users/dto.rs src/app/users/handler.rs \
         admin/src/api.ts admin/src/auth.tsx admin/src/App.tsx admin/src/Resource.tsx; do
  cp /tmp/social-regen/$f $f
done

gize sync              # atualiza openapi.json e admin/src/resources.ts; sobram as 24 personalizações
```

- **0.9.11:** o `POST /users/register` aceita o body sem `is_admin`. Quem ainda manda o campo
  continua funcionando; o valor é ignorado.
- **0.9.12:** no admin, um 403 aparece como "You don't have permission to see …" em vez de lista
  vazia, e um token expirado volta para o login.

## Manutenção

```sh
gize doctor   # confere o ambiente e o projeto
gize check    # clippy
gize fmt      # rustfmt
cargo test
```
