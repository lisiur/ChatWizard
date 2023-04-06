export function autoGrowTextarea(element: HTMLTextAreaElement, config?: {
  minHeight?: number;
}) {
  element.style.height = "5px";
  const newHeight = Math.max(element.scrollHeight, config?.minHeight ?? 5);
  element.style.height = newHeight + "px";
}
