<?php

namespace App;

use Doctrine\ORM\Mapping as ORM;

/**
 * @\Doctrine\ORM\Mapping\Table(name="t", indexes={@\Doctrine\ORM\Mapping\Index(name="i", columns={"a", "b"})})
 * @\Doctrine\ORM\Mapping\Entity("a", "b")
 * @\Doctrine\ORM\Mapping\Column(type="string", length=1_000, scale=1.5, nullable=TRUE, options={"x": 1, "y"=FALSE, 'z'=null})
 * @\Doctrine\ORM\Mapping\JoinColumns(@\Doctrine\ORM\Mapping\JoinColumn, @\Doctrine\ORM\Mapping\JoinColumn(), key=\App\Foo::BAR, all=Foo::*, some=Foo::PRE_*)
 * @\Doctrine\ORM\Mapping\OrderBy({1, 2, 3=4, Foo::BAR="x", "k"="v", k2: "v2", {nested, list}, {}})
 * @\Doctrine\ORM\Mapping\UniqueConstraint("say ""hi""", "a" "b", text="multi
 *     line")
 * @\Doctrine\ORM\Mapping\JoinTable(
 *     first="1",
 *     second={
 *         "a",
 *         "b",
 *     },
 * )
 * @\Doctrine\ORM\Mapping\DiscriminatorMap(x=1) and some words
 * @\Doctrine\ORM\Mapping\Embeddable
 * @\Doctrine\ORM\Mapping\ManyToOne()
 * @\Doctrine\ORM\Mapping\ManyToMany('single')
 * @ORM\InverseJoinColumn some text
 */
class A
{
    /**
     * @\Doctrine\ORM\Mapping\Column(type="integer") @see Other
     * @\Doctrine\ORM\Mapping\Id(1) @\Doctrine\ORM\Mapping\GeneratedValue
     * @\Doctrine\ORM\Mapping\CustomIdGenerator(a=1, a=2, 5="five", b)
     */
    public $id;
}
-----
<?php

namespace App;

use Doctrine\ORM\Mapping\Table;
use Doctrine\ORM\Mapping\Index;
use Doctrine\ORM\Mapping\Entity;
use Doctrine\ORM\Mapping\Column;
use Doctrine\ORM\Mapping\JoinColumns;
use Doctrine\ORM\Mapping\JoinColumn;
use Doctrine\ORM\Mapping\OrderBy;
use Doctrine\ORM\Mapping\UniqueConstraint;
use Doctrine\ORM\Mapping\JoinTable;
use Doctrine\ORM\Mapping\DiscriminatorMap;
use Doctrine\ORM\Mapping\Embeddable;
use Doctrine\ORM\Mapping\ManyToOne;
/**
 * @Table(name="t", indexes={@Index(name="i", columns={"a", "b"})})
 * @Entity("a")
 * @Column(type="string", length=1000, scale=1.5, nullable=true, options={"x"=1, "y"=false, 'z'=})
 * @JoinColumns(@JoinColumn, @JoinColumn, key=\App\Foo::BAR, all=Foo::)
 * @OrderBy({1, 2, 3=4, Foo::BAR="x", "k"="v", k2="v2", nestedlist, })
 * @UniqueConstraint("say ")
 * @JoinTable(first="1", second={"a", "b"})
 * @DiscriminatorMap(x=1)
 * @Embeddable
 * @ManyToOne
 * @\Doctrine\ORM\Mapping\ManyToMany('single')
 * @ORM\InverseJoinColumn some text
 */
class A
{
    /**
     * @Column(type="integer") @see Other
     * @\Doctrine\ORM\Mapping\Id(1) @\Doctrine\ORM\Mapping\GeneratedValue
     * @\Doctrine\ORM\Mapping\CustomIdGenerator(a=1, a=2, 5="five", b)
     */
    public $id;
}
