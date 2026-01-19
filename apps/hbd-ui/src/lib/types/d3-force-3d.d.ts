declare module 'd3-force-3d' {
  export interface SimulationNode {
    x?: number;
    y?: number;
    z?: number;
    vx?: number;
    vy?: number;
    vz?: number;
    fx?: number | null;
    fy?: number | null;
    fz?: number | null;
    index?: number;
  }

  export interface SimulationLink<N extends SimulationNode> {
    source: N | string | number;
    target: N | string | number;
    index?: number;
  }

  export interface Simulation<N extends SimulationNode, L extends SimulationLink<N>> {
    tick(iterations?: number): this;
    restart(): this;
    stop(): this;
    nodes(): N[];
    nodes(nodes: N[]): this;
    alpha(): number;
    alpha(alpha: number): this;
    alphaMin(): number;
    alphaMin(min: number): this;
    alphaDecay(): number;
    alphaDecay(decay: number): this;
    alphaTarget(): number;
    alphaTarget(target: number): this;
    velocityDecay(): number;
    velocityDecay(decay: number): this;
    force(name: string): any;
    force(name: string, force: any): this;
    find(x: number, y: number, z?: number, radius?: number): N | undefined;
    on(typenames: string, listener: any): this;
  }

  export function forceSimulation<N extends SimulationNode>(
    nodes?: N[],
    numDimensions?: number
  ): Simulation<N, SimulationLink<N>>;

  export function forceLink<N extends SimulationNode, L extends SimulationLink<N>>(
    links?: L[]
  ): {
    (alpha: number): void;
    links(): L[];
    links(links: L[]): any;
    id(): (node: N, i: number, nodes: N[]) => string | number;
    id(id: (node: N, i: number, nodes: N[]) => string | number): any;
    distance(): number | ((link: L, i: number, links: L[]) => number);
    distance(distance: number | ((link: L, i: number, links: L[]) => number)): any;
    strength(): number | ((link: L, i: number, links: L[]) => number);
    strength(strength: number | ((link: L, i: number, links: L[]) => number)): any;
  };

  export function forceManyBody<N extends SimulationNode>(): {
    (alpha: number): void;
    strength(): number | ((node: N, i: number, nodes: N[]) => number);
    strength(strength: number | ((node: N, i: number, nodes: N[]) => number)): any;
    distanceMin(): number;
    distanceMin(distance: number): any;
    distanceMax(): number;
    distanceMax(distance: number): any;
    theta(): number;
    theta(theta: number): any;
  };

  export function forceCenter<N extends SimulationNode>(
    x?: number,
    y?: number,
    z?: number
  ): {
    (alpha: number): void;
    x(): number;
    x(x: number): any;
    y(): number;
    y(y: number): any;
    z(): number;
    z(z: number): any;
    strength(): number;
    strength(strength: number): any;
  };

  export function forceCollide<N extends SimulationNode>(
    radius?: number | ((node: N, i: number, nodes: N[]) => number)
  ): {
    (alpha: number): void;
    radius(): number | ((node: N, i: number, nodes: N[]) => number);
    radius(radius: number | ((node: N, i: number, nodes: N[]) => number)): any;
    strength(): number;
    strength(strength: number): any;
    iterations(): number;
    iterations(iterations: number): any;
  };

  export function forceX<N extends SimulationNode>(
    x?: number | ((node: N, i: number, nodes: N[]) => number)
  ): {
    (alpha: number): void;
    x(): number | ((node: N, i: number, nodes: N[]) => number);
    x(x: number | ((node: N, i: number, nodes: N[]) => number)): any;
    strength(): number | ((node: N, i: number, nodes: N[]) => number);
    strength(strength: number | ((node: N, i: number, nodes: N[]) => number)): any;
  };

  export function forceY<N extends SimulationNode>(
    y?: number | ((node: N, i: number, nodes: N[]) => number)
  ): {
    (alpha: number): void;
    y(): number | ((node: N, i: number, nodes: N[]) => number);
    y(y: number | ((node: N, i: number, nodes: N[]) => number)): any;
    strength(): number | ((node: N, i: number, nodes: N[]) => number);
    strength(strength: number | ((node: N, i: number, nodes: N[]) => number)): any;
  };

  export function forceZ<N extends SimulationNode>(
    z?: number | ((node: N, i: number, nodes: N[]) => number)
  ): {
    (alpha: number): void;
    z(): number | ((node: N, i: number, nodes: N[]) => number);
    z(z: number | ((node: N, i: number, nodes: N[]) => number)): any;
    strength(): number | ((node: N, i: number, nodes: N[]) => number);
    strength(strength: number | ((node: N, i: number, nodes: N[]) => number)): any;
  };
}
