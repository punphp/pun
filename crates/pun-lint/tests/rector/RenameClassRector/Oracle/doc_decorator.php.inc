<?php

namespace App;

/**
 * @template T
 */
class A
{
    /**
     * Some text @\Doctrine\ORM\Mapping\OneToMany(targetEntity="Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass")
     * @var \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass
     */
    public $p012;

    /**
     * Some text @\Doctrine\ORM\Mapping\OneToMany(targetEntity="Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass")
     * @Doctrine\ORM\Mapping\Column(type="string")
     * @var \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass
     */
    public $p113;

    /**
     * @var \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass some desc
     * more @\Doctrine\ORM\Mapping\OneToMany(targetEntity="Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass") tail
     * @return int
     */
    public $p213;

    /**
     * @T(\Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass, k=v)
     * @var \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass
     */
    public $p314;

    /**
     * @Doctrine\ORM\Mapping\Table x({
     * @Doctrine\ORM\Mapping\Index a
     * @var \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass
     * @Doctrine\ORM\Mapping\Index b})
     * @var \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass
     */
    public $p415;

    /**
     * @Doctrine\ORM\Mapping\JoinTable {@\Doctrine\ORM\Mapping\OneToMany(targetEntity="Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass")}
     * @var \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass
     */
    public $p512;

    /**
     * @\(x)
     * @var \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass
     */
    public $p612;

    /**
     * Foo @\Doctrine\ORM\Mapping\Index(abc
     * @var \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass
     */
    public $p712;

    /**
     * @param $a desc
     * more @\Doctrine\ORM\Mapping\Index(name="x") tail
     * @deprecated since
     * then @\Doctrine\ORM\Mapping\Index(name="y") end
     * @var \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass
     */
    public $p815;

    /**
     * @Doctrine\ORM\Mapping\Table {(
     * @Doctrine\ORM\Mapping\Column x)}
     * @var \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass
     */
    public $p913;

    /**
     * @Doctrine\ORM\Mapping\Table({
     * })
     * text after
     * @var \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass
     */
    public $p1014;

    /**
     * @T(
     * @var \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass
     */
    public $p1114;

    /**
     * Some text @\Doctrine\ORM\Mapping\OneToMany(targetEntity="Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass")
     * @var \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass
     */
    public $p1211;

    /**
     * @var \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass some description
     * @\Doctrine\ORM\Mapping\OneToMany(targetEntity="Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass")
     */
    public $p1217;

    /**
     * @Doctrine\ORM\Mapping\JoinTable @\Doctrine\ORM\Mapping\OneToMany(targetEntity="Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass")
     * @var \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass
     */
    public $p1223;

    /**
     * @Doctrine\ORM\Mapping\JoinTable {(
     * @Doctrine\ORM\Mapping\Column x)}
     * @var \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass
     */
    public $p1230;

    /**
     * @Doctrine\ORM\Mapping\JoinTable @\Doctrine\ORM\Mapping\Column(type="x")
     * @var \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass
     */
    public $p1311;

    /**
     * @Doctrine\ORM\Mapping\JoinTable @\Doctrine\ORM\Mapping\Column
     * @var \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass
     */
    public $p1317;

    /**
     * @var \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass
     * @Doctrine\ORM\Mapping\JoinTable @\Doctrine\ORM\Mapping\Column
     */
    public $p1323;

    /**
     * @Doctrine\ORM\Mapping\JoinTable x
     * @var \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass
     */
    public $p1329;

}
-----
<?php

namespace App;

/**
 * @template T
 */
class A
{
    /**
     * Some text @\Doctrine\ORM\Mapping\OneToMany(targetEntity="Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass")
     * @\Doctrine\ORM\Mapping\OneToMany(targetEntity="Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass")
     * @var \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass
     */
    public $p012;

    /**
     * Some text @\Doctrine\ORM\Mapping\OneToMany(targetEntity="Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass")
     * @Doctrine\ORM\Mapping\Column(type="string")
     * @var \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass
     */
    public $p113;

    /**
     * @var \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass some desc
     * more @\Doctrine\ORM\Mapping\OneToMany(targetEntity="Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass") tail
     * @return int
     * @\Doctrine\ORM\Mapping\OneToMany(targetEntity="Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass")
     * @var \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass some desc
     * mor
     */
    public $p213;

    /**
     * @T(\Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass, k=v)
     * @var \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass
     */
    public $p314;

    /**
     * @Doctrine\ORM\Mapping\Table x({
     * @Doctrine\ORM\Mapping\Index a
     * @var \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass
     * @Doctrine\ORM\Mapping\Index b})
     * @var \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass
     * @var \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass
     */
    public $p415;

    /**
     * @Doctrine\ORM\Mapping\JoinTable {@\Doctrine\ORM\Mapping\OneToMany(targetEntity="Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass")}
     * @\Doctrine\ORM\Mapping\OneToMany(targetEntity="Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass")
     * @var \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass
     */
    public $p512;

    /**
     * @\(x)
     * @var \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass
     */
    public $p612;

    /**
     * Foo @\Doctrine\ORM\Mapping\Index(abc
     * @\Doctrine\ORM\Mapping\Index("abc")
     * @var \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass
     */
    public $p712;

    /**
     * @param $a desc
     * more @\Doctrine\ORM\Mapping\Index(name="x") tail
     * @deprecated since
     * then @\Doctrine\ORM\Mapping\Index(name="y") end
     * @\Doctrine\ORM\Mapping\Index(name="x")
     * @var \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass
     * @param $a desc
     * mor
     * @deprecated since
     * th
     * @\Doctrine\ORM\Mapping\Index(name="y")
     */
    public $p815;

    /**
     * @Doctrine\ORM\Mapping\Table {(
     * @Doctrine\ORM\Mapping\Column x)}
     * @var \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass
     */
    public $p913;

    /**
     * @Doctrine\ORM\Mapping\Table({
     * })
     * text after
     * @var \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass
     */
    public $p1014;

    /**
     * @T(
     * @var \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass
     */
    public $p1114;

    /**
     * Some text @\Doctrine\ORM\Mapping\OneToMany(targetEntity="Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass")
     * @\Doctrine\ORM\Mapping\OneToMany(targetEntity="Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass")
     * @var \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass
     */
    public $p1211;

    /**
     * @var \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass some description
     * @\Doctrine\ORM\Mapping\OneToMany(targetEntity="Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass")
     */
    public $p1217;

    /**
     * @Doctrine\ORM\Mapping\JoinTable @\Doctrine\ORM\Mapping\OneToMany(targetEntity="Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass")
     * @var \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass
     */
    public $p1223;

    /**
     * @Doctrine\ORM\Mapping\JoinTable {(
     * @Doctrine\ORM\Mapping\Column x)}
     * @var \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass
     */
    public $p1230;

    /**
     * @Doctrine\ORM\Mapping\JoinTable @\Doctrine\ORM\Mapping\Column(type="x")
     * @var \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass
     */
    public $p1311;

    /**
     * @Doctrine\ORM\Mapping\JoinTable @\Doctrine\ORM\Mapping\Column
     * @var \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass
     */
    public $p1317;

    /**
     * @var \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass
     * @Doctrine\ORM\Mapping\JoinTable @\Doctrine\ORM\Mapping\Column
     */
    public $p1323;

    /**
     * @Doctrine\ORM\Mapping\JoinTable x
     * @var \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass
     */
    public $p1329;

}
