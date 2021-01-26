export const dateTime = new Intl.DateTimeFormat("ja", {
  weekday: "narrow", //| 'short' | 'long',
  // era: "narrow", //| 'short' | 'long',
  year: "2-digit",
  month: "numeric", //| '2-digit' | 'narrow' | 'short' | 'long',
  day: "numeric", //| '2-digit',
  hour: "numeric", //| '2-digit',
  minute: "numeric", //| '2-digit',
  second: "numeric", //| '2-digit',
  // timeZoneName: 'short' | 'long',

  // Time zone to express it in
  // timeZone: 'Asia/Shanghai',
  // Force 12-hour or 24-hour
  // hour12: true | false,

  // Rarely-used options
  // hourCycle: "h11" | "h12" | "h23" | "h24",
  // formatMatcher: "basic" | "best fit",
});

export const maybeTimestamp = (maybeNum?: string | null) => {
  if (!maybeNum) return undefined;
  try {
    return new Date(maybeNum).getTime();
  } catch {
    return undefined;
  }
};

export const maybeDateString = (maybeDateString?: number | null) => {
  if (!maybeDateString) return undefined;
  try {
    return new Date(maybeDateString).toISOString().split("T")[0];
  } catch {
    return undefined;
  }
};
