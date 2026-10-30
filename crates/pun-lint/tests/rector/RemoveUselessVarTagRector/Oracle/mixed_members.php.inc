<?php

namespace App;

use App\Models\User;
use DateTimeInterface as When;

final class MixedMembers
{
    /** @var int */
    private int $count;

    /**
     * @var int|null
     */
    private ?int $maybe;

    /** @var int[] */
    private array $list;

    /** @var non-empty-string */
    private string $name;

    /** @var User */
    private User $user;

    /** @var When */
    private When $when;

    /** @var \App\Models\User|null */
    private ?User $owner;

    /** @var string the label */
    private string $label;

    /** @var int */
    // a note
    private int $noted;

    /**
     * Some text.
     *
     * @var int
     */
    private int $described;

    /** @var string|int */
    private int|string $union;

    /** @var int|string */
    private int|string $flipped;

    /** @var mixed */
    private $untyped;

    /** @var string */
    public const A = 'a';

    public function run(): void
    {
        /** @var int $x */
        $x = 5;
        /** @var User $u */
        $u = new User();
        /** @var string $other */
        $y = 'y';
    }
}
-----
<?php

namespace App;

use App\Models\User;
use DateTimeInterface as When;

final class MixedMembers
{
    private int $count;

    private ?int $maybe;

    /** @var int[] */
    private array $list;

    /** @var non-empty-string */
    private string $name;

    private User $user;

    private When $when;

    private ?User $owner;

    /** @var string the label */
    private string $label;

    // a note
    private int $noted;

    /**
     * Some text.
     */
    private int $described;

    private int|string $union;

    private int|string $flipped;

    /** @var mixed */
    private $untyped;

    /** @var string */
    public const A = 'a';

    public function run(): void
    {
        /** @var int $x */
        $x = 5;
        $u = new User();
        /** @var string $other */
        $y = 'y';
    }
}
