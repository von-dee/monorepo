export const addQueryImportsDirective = (
  schema: string,
  externalImports: string[],
  mutation: boolean
): string => {
  if (!externalImports.length) {
    return schema;
  }

  // Append the @imports(...) directive to the query type
  const typeCapture = mutation
    ? /type\s+Mutation\s+([^{]*)\s*{/g
    : /type\s+Query\s+([^{]*)\s*{/g;

  const importedTypes = `${externalImports
    .map((type) => `\"${type}\"`)
    .join(",\n    ")}`;

  const replacementQueryStr = `type ${
    mutation ? "Mutation" : "Query"
  } $1@imports(
  types: [
    ${importedTypes}
  ]
) {`;

  return schema.replace(typeCapture, replacementQueryStr);
}