// @ts-ignore
import { expect, test } from "bun:test";
import { findOverlap, merge } from "./overlap";

// biome-ignore format: highlight overlap
test("overlap_6",() => {
    const a = [6, 8, 4, 6, 7, 7, 1, 2, 7, 3, 3];
    const b =                [7, 1, 2, 7, 3, 3, 1, 1, 1];
    expect(findOverlap(2, a, b)).toBe(6);
})

// biome-ignore format: highlight overlap
test("merged_6",() => {
    const a =                         [6, 8, 4, 6, 7, 7, 1, 2, 7, 3, 3];
    const b =                                        [7, 1, 2, 7, 3, 3, 1, 1, 1];
    expect(merge(a, b)).toStrictEqual([6, 8, 4, 6, 7, 7, 1, 2, 7, 3, 3, 1, 1, 1]);
})

test("merged_on_threshold", () => {
    const a = [1, 2, 3, 4];
    const b = [3, 4, 5, 6];
    expect(merge(a, b)).toStrictEqual([1, 2, 3, 4, 5, 6]);
});

test("merged_none", () => {
    const a = [5, 5, 1, 2];
    const b = [5, 1, 7, 8];
    expect(merge(a, b)).toStrictEqual([5, 5, 1, 2, 5, 1, 7, 8]);
});

test("overlap_all", () => {
    const a = [5, 5, 5, 5];
    const b = [5, 5, 5, 5];
    expect(findOverlap(2, a, b)).toBe(4);
});

test("merged_exact", () => {
    const a = [5, 5, 5, 5];
    const b = [5, 5, 5, 5];
    expect(merge(a, b)).toStrictEqual([5, 5, 5, 5]);
});

test("merged_all_a", () => {
    const a = [5, 5, 5];
    const b = [5, 5, 5, 5];
    expect(merge(a, b)).toStrictEqual([5, 5, 5, 5]);
});

test("merged_all_b", () => {
    const a = [5, 5, 5, 5];
    const b = [5, 5, 5];
    expect(merge(a, b)).toStrictEqual([5, 5, 5, 5]);
});

test("repeating_pattern", () => {
    const a = [1, 2, 1, 2];
    const b = [1, 2, 1, 2];
    expect(merge(a, b)).toStrictEqual([1, 2, 1, 2]);
});

test("long_array_iter", () => {
    const a = Array(5_000).fill(0);
    const b = Array(5_000).fill(5);
    expect(merge(a, b).length).toBe(10_000);
});
