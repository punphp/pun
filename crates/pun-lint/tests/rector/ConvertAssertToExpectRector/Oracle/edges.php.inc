<?php

class BarTest extends TestCase
{
    public function test_a(): void
    {
        $this->assertIsIterable($a);
        $this->assertIsScalar($a);
        $this->assertIsResource($a);
        $this->assertInfinite($a);
        $this->assertFileIsReadable($a);
        $this->assertDirectoryIsReadable($a);
        $this->assertNotTrue($a);
        $this->assertNotFalse($a);
    }
}
-----
<?php

class BarTest extends TestCase
{
    public function test_a(): void
    {
        expect($a)->toBeIterable();
        expect($a)->toBeScalar();
        expect($a)->toBeResource();
        expect($a)->toBeInfinite();
        expect($a)->toBeReadableFile();
        expect($a)->toBeReadableDirectory();
        expect($a)->not->toBeTrue();
        expect($a)->not->toBeFalse();
    }
}
