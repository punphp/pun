<?php
class SomeTest extends \PHPUnit\Framework\TestCase
{
    public function testB()
    {
        $foos = [];
        // check all
        foreach ($foos as $foo) {
            $this->assertInstanceOf(\SplFileInfo::class, $foo);
        }
        if ($x) foreach ($foos as $foo) {
            $this->assertInstanceOf(\SplFileInfo::class, $foo);
        }
        foreach ($foos as $foo) {
            // inner
            $this->assertInstanceOf(\SplFileInfo::class, $foo);
        }
        foreach ($foos as $foo)
            $this->assertInstanceOf(\SplFileInfo::class, $foo, 'a', 'b');
        foreach ($foos as $foo):
            Foo::assertInstanceOf(\SplFileInfo::class, $foo);
        endforeach;
        foreach ($foos as $foo) {
            assertInstanceOf(\SplFileInfo::class, $foo);
        }
    }
}
-----
<?php
class SomeTest extends \PHPUnit\Framework\TestCase
{
    public function testB()
    {
        $foos = [];
        $this->assertContainsOnlyInstancesOf(\SplFileInfo::class, $foos);
        if ($x) $this->assertContainsOnlyInstancesOf(\SplFileInfo::class, $foos);
        $this->assertContainsOnlyInstancesOf(\SplFileInfo::class, $foos);
        $this->assertContainsOnlyInstancesOf(\SplFileInfo::class, $foos);
        Foo::assertContainsOnlyInstancesOf(\SplFileInfo::class, $foos);
        foreach ($foos as $foo) {
            assertInstanceOf(\SplFileInfo::class, $foo);
        }
    }
}
