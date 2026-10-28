<?php

namespace App;

use Illuminate\Contracts\Foundation\Application;
use Illuminate\Contracts\Foundation\Application as App2;

class B
{
    /**
     * The application instance.
     *
     * @var \Illuminate\Contracts\Foundation\Application
     */
    protected $app;

    /**
     * The config.
     *
     * @var array
     */
    protected $config;

    /**
     * @var array<string, mixed>
     */
    protected $options;

    /**
     * The name.
     *
     * @var string|null
     */
    protected $name;

    /**
     * @var int
     */
    protected $count;

    /**
     * The alias.
     *
     * @var App2
     */
    protected $aliased;

    /**
     * Some callback.
     *
     * @var callable
     */
    protected $callback;

    /**
     * Some callback.
     *
     * @var \Closure|null
     */
    protected $closure;

    /** @var string */
    protected $inline;

    /**
     * @var string
     * @deprecated
     */
    protected $deprecated;

    /**
     * @var string[]
     */
    protected $list;

    /**
     * @var bool
     */
    public $pub;

    /**
     * Description only.
     */
    public $pub2;

    /**
     * @var mixed
     */
    public $pub3;

    /**
     * @var Unknown
     */
    protected $unknown;

    /**
     * @var \Illuminate\Contracts\Foundation\Application|\Closure
     */
    protected $wider;

    public function __construct(Application $app, array $config, array $options, ?string $name, int $count, App2 $aliased, callable $callback, ?\Closure $closure, string $inline, string $deprecated, array $list, bool $pub, int $pub2, int $pub3, int $unknown, Application $wider)
    {
        $this->app = $app;
        $this->config = $config;
        $this->options = $options;
        $this->name = $name;
        $this->count = $count;
        $this->aliased = $aliased;
        $this->callback = $callback;
        $this->closure = $closure;
        $this->inline = $inline;
        $this->deprecated = $deprecated;
        $this->list = $list;
        $this->pub = $pub;
        $this->pub2 = $pub2;
        $this->pub3 = $pub3;
        $this->unknown = $unknown;
        $this->wider = $wider;
    }
}
-----
<?php

namespace App;

use Illuminate\Contracts\Foundation\Application;
use Illuminate\Contracts\Foundation\Application as App2;

class B
{
    /**
     * The application instance.
     *
     * @var \Illuminate\Contracts\Foundation\Application
     */
    protected $app;

    /**
     * The config.
     */
    protected array $config;

    /**
     * @var array<string, mixed>
     */
    protected array $options;

    /**
     * The name.
     */
    protected ?string $name;

    protected int $count;

    /**
     * The alias.
     *
     * @var App2
     */
    protected $aliased;

    /**
     * Some callback.
     *
     * @var callable
     */
    protected $callback;

    /**
     * Some callback.
     */
    protected ?\Closure $closure;

    protected string $inline;

    /**
     * @deprecated
     */
    protected string $deprecated;

    /**
     * @var string[]
     */
    protected array $list;

    /**
     * @var bool
     */
    public $pub;

    /**
     * Description only.
     * @var int
     */
    public $pub2;

    /**
     * @var int
     */
    public $pub3;

    /**
     * @var Unknown
     */
    protected $unknown;

    /**
     * @var \Illuminate\Contracts\Foundation\Application|\Closure
     */
    protected \Illuminate\Contracts\Foundation\Application $wider;

    public function __construct(Application $app, array $config, array $options, ?string $name, int $count, App2 $aliased, callable $callback, ?\Closure $closure, string $inline, string $deprecated, array $list, bool $pub, int $pub2, int $pub3, int $unknown, Application $wider)
    {
        $this->app = $app;
        $this->config = $config;
        $this->options = $options;
        $this->name = $name;
        $this->count = $count;
        $this->aliased = $aliased;
        $this->callback = $callback;
        $this->closure = $closure;
        $this->inline = $inline;
        $this->deprecated = $deprecated;
        $this->list = $list;
        $this->pub = $pub;
        $this->pub2 = $pub2;
        $this->pub3 = $pub3;
        $this->unknown = $unknown;
        $this->wider = $wider;
    }
}
