<?php

class BazTest extends TestCase
{
    public function test_a(): void
    {
        $this->assertFileIsWritable($a);
        $this->assertDirectoryIsWritable($a);
    }
}
-----
<?php

class BazTest extends TestCase
{
    public function test_a(): void
    {
        expect($a)->toBeWritableFile();
        expect($a)->toBeWritableDirectory();
    }
}
