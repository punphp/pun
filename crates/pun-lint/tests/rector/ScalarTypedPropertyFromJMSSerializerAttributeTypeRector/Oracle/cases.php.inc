<?php

namespace App;

use Doctrine\ORM\Mapping as ORM;
use JMS\Serializer\Annotation as Serializer;
use JMS\Serializer\Annotation\Type;

final class Scalars
{
    #[Type('integer')]
    private $count;
    #[Type('INT')]
    private $upper;
    #[Type('boolean')]
    private $flag;
    #[Type('true')]
    private $yes;
    #[Type('double')]
    private $ratio;
    #[Type('non-empty-string')]
    private $label;
    #[Type('array')]
    private $items;
    #[Type('non-empty-array')]
    private $nonEmpty;
    #[Type('iterable')]
    private $iter;
    #[Type('object')]
    private $obj;
    #[Type('positive-int')]
    private $positive;
    #[Type('mixed')]
    private $mixed;
    #[Type('list')]
    private $list;
    #[Type('array-key')]
    private $key;
    #[Type('callable')]
    private $callable;
    #[Type('DateTime')]
    private $date;
    #[Type(name: 'string')]
    private $named;
    #[Serializer\Type('int')]
    private $aliased;
    /**
     * @var float
     */
    #[Type('string')]
    private $amount;
    /**
     * @var string The price
     */
    #[Type('float')]
    private $price;
    #[Type('string')]
    private $withDefault = 'x';
    #[Type('string')]
    #[ORM\OneToMany(targetEntity: Scalars::class)]
    private $toMany;
    #[Type('string')]
    public $public;
    #[Type('string')]
    protected $prot;
}
-----
<?php

namespace App;

use Doctrine\ORM\Mapping as ORM;
use JMS\Serializer\Annotation as Serializer;
use JMS\Serializer\Annotation\Type;

final class Scalars
{
    #[Type('integer')]
    private ?int $count = null;
    #[Type('INT')]
    private ?int $upper = null;
    #[Type('boolean')]
    private ?bool $flag = null;
    #[Type('true')]
    private ?bool $yes = null;
    #[Type('double')]
    private ?float $ratio = null;
    #[Type('non-empty-string')]
    private ?string $label = null;
    #[Type('array')]
    private ?array $items = null;
    #[Type('non-empty-array')]
    private ?array $nonEmpty = null;
    #[Type('iterable')]
    private ?iterable $iter = null;
    #[Type('object')]
    private ?object $obj = null;
    #[Type('positive-int')]
    private ?int $positive = null;
    #[Type('mixed')]
    private $mixed;
    #[Type('list')]
    private ?array $list = null;
    #[Type('array-key')]
    private $key;
    #[Type('callable')]
    private $callable;
    #[Type('DateTime')]
    private $date;
    #[Type(name: 'string')]
    private ?string $named = null;
    #[Serializer\Type('int')]
    private ?int $aliased = null;
    #[Type('string')]
    private ?float $amount = null;
    #[Type('float')]
    private ?string $price = null;
    #[Type('string')]
    private $withDefault = 'x';
    #[Type('string')]
    #[ORM\OneToMany(targetEntity: Scalars::class)]
    private $toMany;
    #[Type('string')]
    public ?string $public = null;
    #[Type('string')]
    protected ?string $prot = null;
}
