<?php

namespace App;

use Doctrine\ORM\Mapping as ORM;

/** */
class A
{
    /**
     * @Doctrine\ORM\Mapping\Column(a="x", "y")
     * @var \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass
     */
    public $p0;

    /**
     * @Doctrine\ORM\Mapping\Column(a=1, a=2, b=true)
     * @var \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass
     */
    public $p1;

    /**
     * @Doctrine\ORM\Mapping\Column(name="abc
     * def", type="x")
     * @var \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass
     */
    public $p2;

    /**
     * @Doctrine\ORM\Mapping\Column(name="abc def ghi")
     * @var \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass
     */
    public $p3;

    /**
     * @Doctrine\ORM\Mapping\Column({"a", "b"}, x={1: "a", "k": "v", A::B: 1})
     * @var \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass
     */
    public $p4;

    /**
     * @Doctrine\ORM\Mapping\Column(x={a=1, b: 2, })
     * @var \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass
     */
    public $p5;

    /**
     * @Doctrine\ORM\Mapping\Column(x={
     *     "a",
     *     "b"
     * })
     * @var \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass
     */
    public $p6;

    /**
     * @Some_Thing(x)
     * @var \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass
     */
    public $p7;

    /**
     * @phpstan-import-type Foo
     * @var \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass
     */
    public $p8;

    /**
     * @phpstan-type A int desc
     * @var \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass
     */
    public $p9;

    /**
     * @phpstan-type B array{a: int}
     * @var \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass
     */
    public $p10;

    /**
     * @Foo a @var int x
     * @var \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass
     */
    public $p11;

    /**
     * @Foo a @Bar(=)
     * @var \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass
     */
    public $p12;

    /**
     * @Foo a @Bar(x) tail
     * @var \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass
     */
    public $p13;

    /**
     * @Foo([1])
     * @var \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass
     */
    public $p14;

    /**
     * @Foo(x=[1, A, B::C, 'k' => 2])
     * @var \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass
     */
    public $p15;

    /**
     * @Foo(array(1, 2))
     * @var \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass
     */
    public $p16;

    /**
     * @Foo(A::B* )
     * @var \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass
     */
    public $p17;

    /**
     * @Foo(A::*)
     * @var \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass
     */
    public $p18;

    /**
     * @Foo({A::B=1})
     * @var \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass
     */
    public $p19;

    /**
     * @Foo({1=2, "s"="t", 'q'=3, c:4})
     * @var \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass
     */
    public $p20;

    /**
     * @Foo(@Bar, @Baz(1))
     * @var \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass
     */
    public $p21;

    /**
     * @Foo(1.5, -1, 0x1F, 1_000)
     * @var \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass
     */
    public $p22;

    /**
     * @Doctrine\ORM\Mapping\Column(x=)
     * @var \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass
     */
    public $p23;

    /**
     * @Doctrine\ORM\Mapping\Column(x={a)
     * @var \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass
     */
    public $p24;

    /**
     * @Doctrine\ORM\Mapping\Column()
     * @var \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass
     */
    public $p25;

    /**
     * @Doctrine\ORM\Mapping\Column
     * @var \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass
     */
    public $p26;

    /**
     * @Doctrine\ORM\Mapping\Column(type=Doctrine\ORM\Mapping\Id::class)
     * @var \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass
     */
    public $p27;

    /**
     * @Doctrine\ORM\Mapping\Column(type=true, n=null, f=false)
     * @var \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass
     */
    public $p28;

}
-----
<?php

namespace App;

use Doctrine\ORM\Mapping\Column;
use Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass;
/** */
class A
{
    /**
     * @Column(a="x")
     * @var NewClass
     */
    public $p0;

    /**
     * @Column(a=2, b=true)
     * @var NewClass
     */
    public $p1;

    /**
     * @Column(name="abc
     def", type="x")
     * @var NewClass
    */
    public $p2;

    /**
     * @Column(name="abc def ghi")
     * @var NewClass
     */
    public $p3;

    /**
     * @Column({"a", "b"}, x={1="a", "k"="v", A::B=1})
     * @var NewClass
     */
    public $p4;

    /**
     * @Column(x={a=1, b=2})
     * @var NewClass
     */
    public $p5;

    /**
     * @Column(x={"a", "b"})
     * @var NewClass
     */
    public $p6;

    /**
     * @Some_Thing(x)
     * @var NewClass
     */
    public $p7;

    /**
     * @phpstan-import-type Foo
     * @var NewClass
     */
    public $p8;

    /**
     * @phpstan-type A int desc
     * @var NewClass
     */
    public $p9;

    /**
     * @phpstan-type B array{a: int}
     * @var NewClass
     */
    public $p10;

    /**
     * @Foo a @var int x
     * @var NewClass
     */
    public $p11;

    /**
     * @Foo a @Bar(=)
     * @var \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass
     */
    public $p12;

    /**
     * @Foo a @Bar(x) tail
     * @var \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass
     */
    public $p13;

    /**
     * @Foo([1])
     * @var NewClass
     */
    public $p14;

    /**
     * @Foo(x=[1, A, B::C, 'k' => 2])
     * @var NewClass
     */
    public $p15;

    /**
     * @Foo(array(1, 2))
     * @var NewClass
     */
    public $p16;

    /**
     * @Foo(A::B* )
     * @var NewClass
     */
    public $p17;

    /**
     * @Foo(A::*)
     * @var NewClass
     */
    public $p18;

    /**
     * @Foo({A::B=1})
     * @var NewClass
     */
    public $p19;

    /**
     * @Foo({1=2, "s"="t", 'q'=3, c:4})
     * @var NewClass
     */
    public $p20;

    /**
     * @Foo(@Bar, @Baz(1))
     * @var NewClass
     */
    public $p21;

    /**
     * @Foo(1.5, -1, 0x1F, 1_000)
     * @var NewClass
     */
    public $p22;

    /**
     * @Column(x=))
     * @var NewClass
     */
    public $p23;

    /**
     * @Column(x={a})
     * @var NewClass
     */
    public $p24;

    /**
     * @Column
     * @var NewClass
     */
    public $p25;

    /**
     * @Column
     * @var NewClass
     */
    public $p26;

    /**
     * @Column(type=Doctrine\ORM\Mapping\Id::class)
     * @var NewClass
     */
    public $p27;

    /**
     * @Column(type=true, n=, f=false)
     * @var NewClass
     */
    public $p28;

}
