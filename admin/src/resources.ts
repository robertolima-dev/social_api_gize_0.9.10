import { z } from "zod";

export type FieldKind =
  | "string"
  | "number"
  | "boolean"
  | "uuid"
  | "datetime"
  | "email"
  | "password";

export interface FieldDesc {
  name: string;
  kind: FieldKind;
  /** For a foreign key: the target resource path (e.g. "users"). */
  ref?: string;
}

export interface ResourceDesc {
  name: string;
  path: string;
  fields: FieldDesc[];
  createSchema: z.ZodTypeAny;
}

export const resources: ResourceDesc[] = [
  {
    name: "Comment",
    path: "comments",
    fields: [
      { name: "body", kind: "string" },
      { name: "post_id", kind: "uuid", ref: "posts" },
      { name: "author_id", kind: "uuid", ref: "users" },
    ],
    createSchema: z.object({
      body: z.string().min(1),
      post_id: z.string().uuid("Choose one"),
      author_id: z.string().uuid("Choose one"),
    }),
  },
  {
    name: "Follow",
    path: "follows",
    fields: [
      { name: "follower_id", kind: "uuid", ref: "users" },
      { name: "followee_id", kind: "uuid", ref: "users" },
    ],
    createSchema: z.object({
      follower_id: z.string().uuid("Choose one"),
      followee_id: z.string().uuid("Choose one"),
    }),
  },
  {
    name: "Hashtag",
    path: "hashtags",
    fields: [
      { name: "name", kind: "string" },
    ],
    createSchema: z.object({
      name: z.string().min(1),
    }),
  },
  {
    name: "Like",
    path: "likes",
    fields: [
      { name: "post_id", kind: "uuid", ref: "posts" },
      { name: "user_id", kind: "uuid", ref: "users" },
    ],
    createSchema: z.object({
      post_id: z.string().uuid("Choose one"),
      user_id: z.string().uuid("Choose one"),
    }),
  },
  {
    name: "Message",
    path: "messages",
    fields: [
      { name: "body", kind: "string" },
      { name: "read", kind: "boolean" },
      { name: "sender_id", kind: "uuid", ref: "users" },
      { name: "recipient_id", kind: "uuid", ref: "users" },
    ],
    createSchema: z.object({
      body: z.string().min(1),
      read: z.boolean(),
      sender_id: z.string().uuid("Choose one"),
      recipient_id: z.string().uuid("Choose one"),
    }),
  },
  {
    name: "Post",
    path: "posts",
    fields: [
      { name: "body", kind: "string" },
      { name: "published", kind: "boolean" },
      { name: "author_id", kind: "uuid", ref: "users" },
    ],
    createSchema: z.object({
      body: z.string().min(1),
      published: z.boolean(),
      author_id: z.string().uuid("Choose one"),
    }),
  },
  {
    name: "Profile",
    path: "profiles",
    fields: [
      { name: "bio", kind: "string" },
      { name: "avatar_url", kind: "string" },
      { name: "user_id", kind: "uuid", ref: "users" },
    ],
    createSchema: z.object({
      bio: z.string().min(1),
      avatar_url: z.string().min(1),
      user_id: z.string().uuid("Choose one"),
    }),
  },
  {
    name: "Selfie",
    path: "selfies",
    fields: [
      { name: "image_url", kind: "string" },
      { name: "caption", kind: "string" },
      { name: "author_id", kind: "uuid", ref: "users" },
    ],
    createSchema: z.object({
      image_url: z.string().min(1),
      caption: z.string().min(1),
      author_id: z.string().uuid("Choose one"),
    }),
  },
  {
    name: "User",
    path: "users",
    fields: [
      { name: "name", kind: "string" },
      { name: "email", kind: "email" },
      { name: "password", kind: "password" },
      { name: "is_admin", kind: "boolean" },
    ],
    createSchema: z.object({
      name: z.string().min(1),
      email: z.string().email(),
      password: z.string().min(8),
      is_admin: z.boolean(),
    }),
  },
];
