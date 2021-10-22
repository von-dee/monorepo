import { SchemaResolver } from "./SchemaResolver";

export interface SchemaResolvers {
  external: SchemaResolver;
  local: SchemaResolver;
}
