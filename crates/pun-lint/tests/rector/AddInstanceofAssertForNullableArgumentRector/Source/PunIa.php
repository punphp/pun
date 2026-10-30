<?php

namespace Pun\Ia;

interface Shape
{
}

class Item
{
    public function name(): string
    {
        return '';
    }

    public function next(): ?Item
    {
        return null;
    }

    public function kind(): ?Kind
    {
        return null;
    }

    public function either(): Item|Other|null
    {
        return null;
    }

    public function count(): ?int
    {
        return null;
    }

    public function self(): ?static
    {
        return null;
    }

    public function both(): (Item&Shape)|null
    {
        return null;
    }

    public function sure(): Item
    {
        return $this;
    }
}

class Other
{
}

enum Kind
{
    case A;

    public function label(): string
    {
        return '';
    }
}

class Repo
{
    public function find(): ?Item
    {
        return null;
    }

    public function findUnknown(): ?Missing
    {
        return null;
    }

    public function take(Item $item): void
    {
    }

    public function takeNullable(?Item $item): void
    {
    }

    public function takeAny($item): void
    {
    }

    /** @param Item $item */
    public function takeDoc($item): void
    {
    }

    public function takeKind(Kind $kind): void
    {
    }

    public function takeTwo(int $a, Item $item): void
    {
    }

    /**
     * @template T of Item
     * @param T $item
     */
    public function takeTemplate($item): void
    {
    }

    public function assertItem(Item $item): void
    {
    }

    public function shouldItem(Item $item): void
    {
    }
}
