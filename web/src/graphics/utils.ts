export function circleLayout(i: number, total: number, radius: number) {
  const radians = ((Math.PI * 2) / total) * i;
  const x = radius * Math.cos(radians);
  const y = radius * Math.sin(radians);
  return [x, y];
}

export function getDomain<A>(datas: A[], extractor: (a: A) => number): [number, number];
export function getDomain(datas: number[]): [number, number];
export function getDomain(datas: any, extractor?: (a: any) => number) {
  let min = 10000000;
  let max = -10000000;
  if (extractor) {
    for (let i = 0; i < datas.length; i++) {
      const data = extractor(datas[i]);
      min = Math.min(min, data);
      max = Math.max(max, data);
    }
  } else {
    for (let i = 0; i < datas.length; i++) {
      const data = datas[i];
      min = Math.min(min, data);
      max = Math.max(max, data);
    }
  }
  return [min, max];
}
