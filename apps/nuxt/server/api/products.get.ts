import { PRODUCTS } from "@src/catalog";
import { summary } from "@src/summary";

export default defineEventHandler(() => ({ count: PRODUCTS.length, products: PRODUCTS.map(summary) }));
