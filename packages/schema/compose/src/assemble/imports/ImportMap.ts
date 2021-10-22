import { ImportedObjectDefinition, ImportedQueryDefinition, ImportedEnumDefinition } from "@web3api/schema-parse";
import { Namespaced } from "./Namespaced";

export type ImportMap = Record<
  string, (ImportedObjectDefinition |
    ImportedQueryDefinition |
    ImportedEnumDefinition) &
  Namespaced
>;
