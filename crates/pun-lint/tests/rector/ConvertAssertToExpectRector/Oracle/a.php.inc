<?php

class FooTest extends TestCase
{
    public function test_a(): void
    {
        $this->assertSame(42, $value, 'message');
        $this->assertTrue(/* c */ $x);
        $THIS->assertTrue($x);
        $this->ASSERTTRUE($x);
        $this->assertEquals([
            'a' => 1,
        ], $this->get(
            'b'
        ));
        $x = $this->assertCount(1, ...$items);
        $this->assertEqualsWithDelta(1.0, $f);
        $this->assertSame(expected: 1, actual: $y);
        $this->assertNull(($z));
        $this->assertFileIsReadable($path);
        $this->{'assertTrue'}($x);
        $this?->assertTrue($x);
        static::assertTrue($x);
        $this->assertTrue();
    }
}

it('works', function () {
    $this->assertInstanceOf(User::class, $user);
});
-----
<?php

class FooTest extends TestCase
{
    public function test_a(): void
    {
        expect($value)->toBe(42);
        expect($x)->toBeTrue();
        expect($x)->toBeTrue();
        $this->ASSERTTRUE($x);
        expect($this->get(
            'b'
        ))->toEqual([
            'a' => 1,
        ]);
        $x = expect($items)->toHaveCount(1);
        $this->assertEqualsWithDelta(1.0, $f);
        expect($y)->toBe(1);
        expect($z)->toBeNull();
        expect($path)->toBeReadableFile();
        $this->{'assertTrue'}($x);
        $this?->assertTrue($x);
        static::assertTrue($x);
        $this->assertTrue();
    }
}

it('works', function () {
    expect($user)->toBeInstanceOf(User::class);
});
