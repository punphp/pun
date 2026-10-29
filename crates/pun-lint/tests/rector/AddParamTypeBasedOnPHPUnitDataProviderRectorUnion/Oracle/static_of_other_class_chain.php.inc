<?php

namespace Probe;

use PHPUnit\Framework\Attributes\DataProvider;
use PHPUnit\Framework\TestCase;

class Moment
{
    public static function parse(string $value): static
    {
        return new static();
    }

    public function later(): static
    {
        return $this;
    }

    /**
     * @return $this
     */
    public function add(int $minutes)
    {
        return $this;
    }
}

final class MomentTest extends TestCase
{
    #[DataProvider('added')]
    public function testAdded($value)
    {
    }

    public static function added()
    {
        return [[Moment::parse('now')->add(5)], [300]];
    }

    #[DataProvider('moments')]
    public function testMoment($value)
    {
    }

    public static function moments()
    {
        return [
            [Moment::parse('now')->later()],
            [new \DateTime()],
            [300],
        ];
    }
}
-----
<?php

namespace Probe;

use PHPUnit\Framework\Attributes\DataProvider;
use PHPUnit\Framework\TestCase;

class Moment
{
    public static function parse(string $value): static
    {
        return new static();
    }

    public function later(): static
    {
        return $this;
    }

    /**
     * @return $this
     */
    public function add(int $minutes)
    {
        return $this;
    }
}

final class MomentTest extends TestCase
{
    #[DataProvider('added')]
    public function testAdded(\Probe\Moment|int $value)
    {
    }

    public static function added()
    {
        return [[Moment::parse('now')->add(5)], [300]];
    }

    #[DataProvider('moments')]
    public function testMoment(\Probe\Moment|\DateTime|int $value)
    {
    }

    public static function moments()
    {
        return [
            [Moment::parse('now')->later()],
            [new \DateTime()],
            [300],
        ];
    }
}
