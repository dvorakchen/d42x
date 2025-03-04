export function toYYYYMMDDHHmmss(utc: string) {
  const date = new Date(utc);

  const formatter = new Intl.DateTimeFormat("zh-CN", {
    year: "numeric",
    month: "2-digit",
    day: "2-digit",
    hour: "2-digit",
    minute: "2-digit",
    second: "2-digit",
    hour12: false,
  });

  const localTimeString = formatter.format(date);

  return localTimeString;
}
