import { TYPE_NAME_REGEX } from "./constants/TYPE-NAME-REGEX";

export const parseInterfaces = (
  implementInterfaceStatments: RegExpMatchArray[]
): ImplementationWithInterfaces[] => {
  const implementationsWithInterfaces: ImplementationWithInterfaces[] = [];

  for (const implementMatch of implementInterfaceStatments) {
    const implementStr = implementMatch[1].trim();
    const typeCapture = new RegExp(`type\\s+(${TYPE_NAME_REGEX})\\s+`, "g");

    const typeNameMatches = typeCapture.exec(implementMatch[0]);

    if (!typeNameMatches) {
      continue;
    }

    const typeName = typeNameMatches[1];

    const interfaces = [
      ...implementStr.matchAll(new RegExp(`(${TYPE_NAME_REGEX})(&\s+)*`, "g")),
    ].map((x) => x[0]);

    implementationsWithInterfaces.push({
      typeName,
      interfaces,
    });
  }

  return implementationsWithInterfaces;
}
