export interface Product {
  slug: string;
  name: string;
  priceCents: number;
  rating: number;
  stock: number;
  tags: string[];
  blurb: string;
}

export const PRODUCTS: Product[] = [
  {slug: "alpine-pack", name: "Alpine Pack", priceCents: 1999, rating: 3.0, stock: 0, tags: ["outdoor", "daily"], blurb: "The Alpine pack is built for long days out and packs down small when you are done with it."},
  {slug: "alpine-jacket", name: "Alpine Jacket", priceCents: 2736, rating: 3.7, stock: 13, tags: ["travel", "rain"], blurb: "The Alpine jacket is built for long days out and packs down small when you are done with it."},
  {slug: "alpine-lantern", name: "Alpine Lantern", priceCents: 3473, rating: 4.4, stock: 26, tags: ["camp", "winter"], blurb: "The Alpine lantern is built for long days out and packs down small when you are done with it."},
  {slug: "alpine-bottle", name: "Alpine Bottle", priceCents: 4210, rating: 3.0, stock: 39, tags: ["daily", "light"], blurb: "The Alpine bottle is built for long days out and packs down small when you are done with it."},
  {slug: "harbor-pack", name: "Harbor Pack", priceCents: 4947, rating: 3.7, stock: 12, tags: ["rain", "classic"], blurb: "The Harbor pack is built for long days out and packs down small when you are done with it."},
  {slug: "harbor-jacket", name: "Harbor Jacket", priceCents: 5684, rating: 4.4, stock: 25, tags: ["winter", "outdoor"], blurb: "The Harbor jacket is built for long days out and packs down small when you are done with it."},
  {slug: "harbor-lantern", name: "Harbor Lantern", priceCents: 6421, rating: 3.0, stock: 38, tags: ["light", "travel"], blurb: "The Harbor lantern is built for long days out and packs down small when you are done with it."},
  {slug: "harbor-bottle", name: "Harbor Bottle", priceCents: 7158, rating: 3.7, stock: 11, tags: ["classic", "camp"], blurb: "The Harbor bottle is built for long days out and packs down small when you are done with it."},
  {slug: "cedar-pack", name: "Cedar Pack", priceCents: 7895, rating: 4.4, stock: 24, tags: ["outdoor", "daily"], blurb: "The Cedar pack is built for long days out and packs down small when you are done with it."},
  {slug: "cedar-jacket", name: "Cedar Jacket", priceCents: 8632, rating: 3.0, stock: 37, tags: ["travel", "rain"], blurb: "The Cedar jacket is built for long days out and packs down small when you are done with it."},
  {slug: "cedar-lantern", name: "Cedar Lantern", priceCents: 9369, rating: 3.7, stock: 10, tags: ["camp", "winter"], blurb: "The Cedar lantern is built for long days out and packs down small when you are done with it."},
  {slug: "cedar-bottle", name: "Cedar Bottle", priceCents: 10106, rating: 4.4, stock: 23, tags: ["daily", "light"], blurb: "The Cedar bottle is built for long days out and packs down small when you are done with it."},
  {slug: "summit-pack", name: "Summit Pack", priceCents: 10843, rating: 3.0, stock: 36, tags: ["rain", "classic"], blurb: "The Summit pack is built for long days out and packs down small when you are done with it."},
  {slug: "summit-jacket", name: "Summit Jacket", priceCents: 11580, rating: 3.7, stock: 9, tags: ["winter", "outdoor"], blurb: "The Summit jacket is built for long days out and packs down small when you are done with it."},
  {slug: "summit-lantern", name: "Summit Lantern", priceCents: 12317, rating: 4.4, stock: 22, tags: ["light", "travel"], blurb: "The Summit lantern is built for long days out and packs down small when you are done with it."},
  {slug: "summit-bottle", name: "Summit Bottle", priceCents: 13054, rating: 3.0, stock: 35, tags: ["classic", "camp"], blurb: "The Summit bottle is built for long days out and packs down small when you are done with it."},
  {slug: "meadow-pack", name: "Meadow Pack", priceCents: 13791, rating: 3.7, stock: 8, tags: ["outdoor", "daily"], blurb: "The Meadow pack is built for long days out and packs down small when you are done with it."},
  {slug: "meadow-jacket", name: "Meadow Jacket", priceCents: 14528, rating: 4.4, stock: 21, tags: ["travel", "rain"], blurb: "The Meadow jacket is built for long days out and packs down small when you are done with it."},
  {slug: "meadow-lantern", name: "Meadow Lantern", priceCents: 15265, rating: 3.0, stock: 34, tags: ["camp", "winter"], blurb: "The Meadow lantern is built for long days out and packs down small when you are done with it."},
  {slug: "meadow-bottle", name: "Meadow Bottle", priceCents: 16002, rating: 3.7, stock: 7, tags: ["daily", "light"], blurb: "The Meadow bottle is built for long days out and packs down small when you are done with it."},
  {slug: "granite-pack", name: "Granite Pack", priceCents: 16739, rating: 4.4, stock: 20, tags: ["rain", "classic"], blurb: "The Granite pack is built for long days out and packs down small when you are done with it."},
  {slug: "granite-jacket", name: "Granite Jacket", priceCents: 17476, rating: 3.0, stock: 33, tags: ["winter", "outdoor"], blurb: "The Granite jacket is built for long days out and packs down small when you are done with it."},
  {slug: "granite-lantern", name: "Granite Lantern", priceCents: 18213, rating: 3.7, stock: 6, tags: ["light", "travel"], blurb: "The Granite lantern is built for long days out and packs down small when you are done with it."},
  {slug: "granite-bottle", name: "Granite Bottle", priceCents: 18950, rating: 4.4, stock: 19, tags: ["classic", "camp"], blurb: "The Granite bottle is built for long days out and packs down small when you are done with it."},
  {slug: "willow-pack", name: "Willow Pack", priceCents: 19687, rating: 3.0, stock: 32, tags: ["outdoor", "daily"], blurb: "The Willow pack is built for long days out and packs down small when you are done with it."},
  {slug: "willow-jacket", name: "Willow Jacket", priceCents: 2424, rating: 3.7, stock: 5, tags: ["travel", "rain"], blurb: "The Willow jacket is built for long days out and packs down small when you are done with it."},
  {slug: "willow-lantern", name: "Willow Lantern", priceCents: 3161, rating: 4.4, stock: 18, tags: ["camp", "winter"], blurb: "The Willow lantern is built for long days out and packs down small when you are done with it."},
  {slug: "willow-bottle", name: "Willow Bottle", priceCents: 3898, rating: 3.0, stock: 31, tags: ["daily", "light"], blurb: "The Willow bottle is built for long days out and packs down small when you are done with it."},
  {slug: "copper-pack", name: "Copper Pack", priceCents: 4635, rating: 3.7, stock: 4, tags: ["rain", "classic"], blurb: "The Copper pack is built for long days out and packs down small when you are done with it."},
  {slug: "copper-jacket", name: "Copper Jacket", priceCents: 5372, rating: 4.4, stock: 17, tags: ["winter", "outdoor"], blurb: "The Copper jacket is built for long days out and packs down small when you are done with it."},
  {slug: "copper-lantern", name: "Copper Lantern", priceCents: 6109, rating: 3.0, stock: 30, tags: ["light", "travel"], blurb: "The Copper lantern is built for long days out and packs down small when you are done with it."},
  {slug: "copper-bottle", name: "Copper Bottle", priceCents: 6846, rating: 3.7, stock: 3, tags: ["classic", "camp"], blurb: "The Copper bottle is built for long days out and packs down small when you are done with it."},
  {slug: "tidal-pack", name: "Tidal Pack", priceCents: 7583, rating: 4.4, stock: 16, tags: ["outdoor", "daily"], blurb: "The Tidal pack is built for long days out and packs down small when you are done with it."},
  {slug: "tidal-jacket", name: "Tidal Jacket", priceCents: 8320, rating: 3.0, stock: 29, tags: ["travel", "rain"], blurb: "The Tidal jacket is built for long days out and packs down small when you are done with it."},
  {slug: "tidal-lantern", name: "Tidal Lantern", priceCents: 9057, rating: 3.7, stock: 2, tags: ["camp", "winter"], blurb: "The Tidal lantern is built for long days out and packs down small when you are done with it."},
  {slug: "tidal-bottle", name: "Tidal Bottle", priceCents: 9794, rating: 4.4, stock: 15, tags: ["daily", "light"], blurb: "The Tidal bottle is built for long days out and packs down small when you are done with it."},
  {slug: "juniper-pack", name: "Juniper Pack", priceCents: 10531, rating: 3.0, stock: 28, tags: ["rain", "classic"], blurb: "The Juniper pack is built for long days out and packs down small when you are done with it."},
  {slug: "juniper-jacket", name: "Juniper Jacket", priceCents: 11268, rating: 3.7, stock: 1, tags: ["winter", "outdoor"], blurb: "The Juniper jacket is built for long days out and packs down small when you are done with it."},
  {slug: "juniper-lantern", name: "Juniper Lantern", priceCents: 12005, rating: 4.4, stock: 14, tags: ["light", "travel"], blurb: "The Juniper lantern is built for long days out and packs down small when you are done with it."},
  {slug: "juniper-bottle", name: "Juniper Bottle", priceCents: 12742, rating: 3.0, stock: 27, tags: ["classic", "camp"], blurb: "The Juniper bottle is built for long days out and packs down small when you are done with it."},
  {slug: "ember-pack", name: "Ember Pack", priceCents: 13479, rating: 3.7, stock: 0, tags: ["outdoor", "daily"], blurb: "The Ember pack is built for long days out and packs down small when you are done with it."},
  {slug: "ember-jacket", name: "Ember Jacket", priceCents: 14216, rating: 4.4, stock: 13, tags: ["travel", "rain"], blurb: "The Ember jacket is built for long days out and packs down small when you are done with it."},
  {slug: "ember-lantern", name: "Ember Lantern", priceCents: 14953, rating: 3.0, stock: 26, tags: ["camp", "winter"], blurb: "The Ember lantern is built for long days out and packs down small when you are done with it."},
  {slug: "ember-bottle", name: "Ember Bottle", priceCents: 15690, rating: 3.7, stock: 39, tags: ["daily", "light"], blurb: "The Ember bottle is built for long days out and packs down small when you are done with it."},
  {slug: "prairie-pack", name: "Prairie Pack", priceCents: 16427, rating: 4.4, stock: 12, tags: ["rain", "classic"], blurb: "The Prairie pack is built for long days out and packs down small when you are done with it."},
  {slug: "prairie-jacket", name: "Prairie Jacket", priceCents: 17164, rating: 3.0, stock: 25, tags: ["winter", "outdoor"], blurb: "The Prairie jacket is built for long days out and packs down small when you are done with it."},
  {slug: "prairie-lantern", name: "Prairie Lantern", priceCents: 17901, rating: 3.7, stock: 38, tags: ["light", "travel"], blurb: "The Prairie lantern is built for long days out and packs down small when you are done with it."},
  {slug: "prairie-bottle", name: "Prairie Bottle", priceCents: 18638, rating: 4.4, stock: 11, tags: ["classic", "camp"], blurb: "The Prairie bottle is built for long days out and packs down small when you are done with it."},
];
