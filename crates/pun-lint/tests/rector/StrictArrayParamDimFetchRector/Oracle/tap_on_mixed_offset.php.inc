<?php

namespace Probe;

use PHPUnit\Framework\TestCase;

class Resource
{
    /**
     * @return array|\JsonSerializable
     */
    public function toArray()
    {
        return [];
    }
}

/**
 * @template TValue
 *
 * @param  TValue  $value
 * @param  (callable(TValue): mixed)|null  $callback
 * @return TValue
 */
function tap($value, $callback = null)
{
    return $value;
}

final class ResourceTest extends TestCase
{
    public function testAsserted()
    {
        tap((new Resource())->toArray(), function ($rows) {
            $this->assertIsArray($rows);
            $this->assertSame(['id' => 5], $rows[0]);
        });
    }

    public function testNotAsserted()
    {
        tap((new Resource())->toArray(), function ($rows) {
            $this->assertSame(['id' => 5], $rows[0]);
        });
    }

    public function testMixed($value)
    {
        tap($value->toArray(), function ($rows) {
            $this->assertSame(['id' => 5], $rows[0]);
        });
    }
}
