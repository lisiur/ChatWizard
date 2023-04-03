export async function sleep(ms: number = 16.67) {
  return new Promise((r) => setTimeout(r, ms));
}
