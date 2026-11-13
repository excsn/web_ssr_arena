import type { APIRoute } from "astro";
import { PRODUCTS } from "@src/catalog";
import { summary } from "@src/summary";

export const GET: APIRoute = () => Response.json({ count: PRODUCTS.length, products: PRODUCTS.map(summary) });
