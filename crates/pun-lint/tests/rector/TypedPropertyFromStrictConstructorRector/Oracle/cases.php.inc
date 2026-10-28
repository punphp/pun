<?php

namespace App;

use Illuminate\Contracts\Foundation\Application;

interface Repo {}
final class Model {}
enum Status: string { case A = 'a'; }

class A
{
    protected $app;
    public $pub;
    private $count = 0;
    private $flag = false;
    private $items = [];
    private $name;
    private $maybe;
    private $union;
    private $status;
    private $closure;
    private $callable;
    private $obj;
    private $self;
    private $iter;
    private static $static;
    private $a, $b;
    var $old;
    /** @var string Some text */
    private $described;
    /** @var array<string, int> */
    private $generic;
    /** @var Repo|null */
    private $docNull;
    private $twice;
    private $fromCall;
    private $fromProp;
    private $nullDefault = null;
    private $condAssign = 1;
    private $mixedParam;
    private $defaultParam;
    private $intDefaultParam = 5;

    public function __construct(
        Application $app,
        $pub,
        int $count,
        bool $flag,
        array $items,
        string $name,
        ?Model $maybe,
        int|string $union,
        Status $status,
        \Closure $closure,
        callable $callable,
        object $obj,
        self $self,
        iterable $iter,
        string $static,
        string $a,
        string $old,
        string $described,
        array $generic,
        Repo $docNull,
        mixed $mixedParam,
        string $defaultParam = 'x',
        int $intDefaultParam = 6,
    ) {
        $this->app = $app;
        $this->pub = new Model();
        $this->count = $count;
        $this->flag = $flag;
        $this->items = $items;
        $this->name = $name;
        $this->maybe = $maybe;
        $this->union = $union;
        $this->status = $status;
        $this->closure = $closure;
        $this->callable = $callable;
        $this->obj = $obj;
        $this->self = $self;
        $this->iter = $iter;
        self::$static = $static;
        $this->a = $a;
        $this->old = $old;
        $this->described = $described;
        $this->generic = $generic;
        $this->docNull = $docNull;
        $this->twice = 1;
        $this->twice = 'x';
        $this->fromCall = strlen($name);
        $this->fromProp = $this->count;
        $this->nullDefault = $name;
        if ($flag) {
            $this->condAssign = 2;
        }
        $this->mixedParam = $mixedParam;
        $this->defaultParam = $defaultParam;
        $this->intDefaultParam = $intDefaultParam;
    }
}
-----
<?php

namespace App;

use Illuminate\Contracts\Foundation\Application;

interface Repo {}
final class Model {}
enum Status: string { case A = 'a'; }

class A
{
    protected \Illuminate\Contracts\Foundation\Application $app;
    /**
     * @var \App\Model
     */
    public $pub;
    private int $count;
    private bool $flag;
    private array $items;
    private string $name;
    private ?\App\Model $maybe;
    private int|string $union;
    private \App\Status $status;
    private \Closure $closure;
    private $callable;
    private object $obj;
    private self $self;
    private iterable $iter;
    private static string $static;
    private $a, $b;
    /**
     * @var string
     */
    var $old;
    /** @var string Some text */
    private string $described;
    /** @var array<string, int> */
    private array $generic;
    private \App\Repo $docNull;
    private $twice;
    private int $fromCall;
    private int $fromProp;
    private string $nullDefault;
    private $condAssign = 1;
    private $mixedParam;
    private string $defaultParam;
    private int $intDefaultParam;

    public function __construct(
        Application $app,
        $pub,
        int $count,
        bool $flag,
        array $items,
        string $name,
        ?Model $maybe,
        int|string $union,
        Status $status,
        \Closure $closure,
        callable $callable,
        object $obj,
        self $self,
        iterable $iter,
        string $static,
        string $a,
        string $old,
        string $described,
        array $generic,
        Repo $docNull,
        mixed $mixedParam,
        string $defaultParam = 'x',
        int $intDefaultParam = 6,
    ) {
        $this->app = $app;
        $this->pub = new Model();
        $this->count = $count;
        $this->flag = $flag;
        $this->items = $items;
        $this->name = $name;
        $this->maybe = $maybe;
        $this->union = $union;
        $this->status = $status;
        $this->closure = $closure;
        $this->callable = $callable;
        $this->obj = $obj;
        $this->self = $self;
        $this->iter = $iter;
        self::$static = $static;
        $this->a = $a;
        $this->old = $old;
        $this->described = $described;
        $this->generic = $generic;
        $this->docNull = $docNull;
        $this->twice = 1;
        $this->twice = 'x';
        $this->fromCall = strlen($name);
        $this->fromProp = $this->count;
        $this->nullDefault = $name;
        if ($flag) {
            $this->condAssign = 2;
        }
        $this->mixedParam = $mixedParam;
        $this->defaultParam = $defaultParam;
        $this->intDefaultParam = $intDefaultParam;
    }
}
