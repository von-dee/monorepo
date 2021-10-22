import { TYPE_NAME_REGEX } from "../constants/TYPE-NAME-REGEX";

export const resolveInterfaces = (
  schema: string,
  implementationsWithInterfaces: ImplementationWithInterfaces[]
): string => {
  const removeComments = (body: string) => {
    const bodyWithoutComments = body.replace(/"""[^"]*"""\s*/g, "");
    return bodyWithoutComments;
  };

  if (!implementationsWithInterfaces.length) {
    return schema;
  }

  const getAllUniqueInterfaces = (): string[] => {
    const allIntefaces = implementationsWithInterfaces
      .map((x) => x.interfaces)
      .reduce((acc, x) => acc.concat(x), []);

    return [...new Set(allIntefaces)];
  };

  const allInterfaces = getAllUniqueInterfaces();
  const interfacesWithBodies: { name: string; body: string }[] = [];

  const typeCapture = new RegExp(
    `type\\s+(${TYPE_NAME_REGEX})[^{]+{([^}]*)}`,
    "g"
  );
  const typeMatches = [...schema.matchAll(typeCapture)];

  for (const interfaceName of allInterfaces) {
    const match = typeMatches.find((x) => x[1] === interfaceName);

    if (!match) {
      continue;
    }

    let body = match[2];
    if (!body) {
      continue;
    }

    body = removeComments(body);

    interfacesWithBodies.push({
      name: interfaceName,
      body: body,
    });
  }

  for (const implementationWithInterfaces of implementationsWithInterfaces) {
    const implementationTypeCapture = new RegExp(
      `(type\\s+${implementationWithInterfaces.typeName}\\s+[^{]*){([^}]*)}`
    );

    const bodiesOfInterfaces = implementationWithInterfaces.interfaces.map(
      (interfaceName) => {
        return interfacesWithBodies
          .find((iwb) => iwb.name === interfaceName)
          ?.body.trim();
      }
    );

    const bodiesOfInterfacesStr = bodiesOfInterfaces
      .filter((x) => x)
      .reduce((acc: string, x: string) => acc + "\n" + x, "");

    schema = schema.replace(
      implementationTypeCapture,
      `$1{$2${bodiesOfInterfacesStr}\n}`
    );
  }

  return schema;
}