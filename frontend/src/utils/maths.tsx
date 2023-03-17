function randomIntFromInterval(min: GLfloat, max: GLfloat) { // min and max included 
    return Math.floor(Math.random() * (max - min + 1) + min)
}

function nFormatter(num: GLfloat, digits: GLfloat) {
    const lookup = [
      { value: 1, symbol: "" },
      { value: 1e3, symbol: "k" },
      { value: 1e6, symbol: "M" },
      { value: 1e9, symbol: "G" },
      { value: 1e12, symbol: "T" },
      { value: 1e15, symbol: "P" },
      { value: 1e18, symbol: "E" }
    ];
    const rx = /\.0+$|(\.[0-9]*[1-9])0+$/;
    const item = lookup.slice().reverse().find(function (item) {
        return num >= item.value;
    });
    return item ? (num / item.value).toFixed(digits).replace(rx, "$1") + item.symbol : "0";
}

function twoDecimals(n: number) {
    const log10 = n ? Math.floor(Math.log10(n)) : 0,
        div = log10 < 0 ? Math.pow(10, 1 - log10) : 100;

    return Math.round(n * div) / div;
}

function formatToString(n: number) {
    const x = twoDecimals(n);
    if (x < 0) {
        const s = x.toLocaleString("en-US");
        return s.slice(1, s.length)
  }
  if (isNaN(x)) return "?";
  else return x.toLocaleString("en-US", {maximumFractionDigits: 9})
}

function formatToString2(n: number) {
  return parseFloat(Math.abs(n).toFixed(2)).toLocaleString("en-US");
}

export { randomIntFromInterval, nFormatter, formatToString, formatToString2 };