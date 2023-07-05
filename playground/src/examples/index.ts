const examples: Record<string, string> = {
  "Hello world": (await import("./helloWorld.panda?raw")).default,
  FizzBuzz: (await import("./fizzbuzz.panda?raw")).default,
};

export function getExample(name: string): string {
  return examples[name];
}

export function getExampleNames(): string[] {
  return Object.keys(examples);
}
