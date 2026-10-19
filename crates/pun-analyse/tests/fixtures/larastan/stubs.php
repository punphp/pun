<?php

// The Laravel 12 declarations (laravel/framework, as Larastan 3.10.0's test application installs it) that
// tests/larastan.rs's fixtures use, cut down to the members the Larastan rules read.

namespace Illuminate\Contracts\Container {
    interface Container
    {
        public function bind($abstract, $concrete = null, $shared = false);

        public function singleton($abstract, $concrete = null);
    }
}

namespace Illuminate\Contracts\Foundation {
    interface Application extends \Illuminate\Contracts\Container\Container
    {
    }
}

namespace Illuminate\Container {
    class Container implements \Illuminate\Contracts\Container\Container
    {
        public function bind($abstract, $concrete = null, $shared = false)
        {
        }

        public function singleton($abstract, $concrete = null)
        {
        }
    }
}

namespace Illuminate\Foundation {
    class Application extends \Illuminate\Container\Container implements \Illuminate\Contracts\Foundation\Application
    {
    }
}

namespace Illuminate\Contracts\Support {
    interface DeferrableProvider
    {
    }
}

namespace Illuminate\Support {
    abstract class ServiceProvider
    {
        /**
         * The application instance.
         *
         * @var \Illuminate\Contracts\Foundation\Application
         */
        protected $app;

        /**
         * @return array
         */
        public function provides()
        {
            return [];
        }
    }
}

namespace Illuminate\Support\Facades {
    abstract class Facade
    {
    }

    class Schema extends Facade
    {
    }

    class Auth extends Facade
    {
    }
}

namespace Illuminate\Contracts\Auth {
    interface Factory
    {
    }

    interface Guard
    {
    }
}

namespace Illuminate\Auth {
    class AuthManager implements \Illuminate\Contracts\Auth\Factory
    {
        public function __call($method, $parameters)
        {
        }
    }
}

namespace Illuminate\Http {
    class Request
    {
        public function user($guard = null)
        {
        }
    }
}

namespace Illuminate\Database\Eloquent {
    abstract class Model
    {
        public static function __callStatic($method, $parameters)
        {
        }

        public function __call($method, $parameters)
        {
        }
    }

    class Builder
    {
        public function where($column, $operator = null, $value = null, $boolean = 'and')
        {
            return $this;
        }
    }
}

namespace Illuminate\Database\Eloquent\Casts {
    class Attribute
    {
        public static function make(?callable $get = null, ?callable $set = null): static
        {
            return new static();
        }
    }
}

namespace Illuminate\Database\Eloquent\Attributes {
    #[\Attribute(\Attribute::TARGET_METHOD)]
    class Scope
    {
    }
}

namespace Illuminate\Foundation\Bus {
    trait Dispatchable
    {
        public static function dispatch(...$arguments)
        {
        }

        public static function dispatchIf($boolean, ...$arguments)
        {
        }

        public static function dispatchUnless($boolean, ...$arguments)
        {
        }

        public static function dispatchSync(...$arguments)
        {
        }

        public static function dispatchAfterResponse(...$arguments)
        {
        }
    }
}

namespace Illuminate\Foundation\Events {
    trait Dispatchable
    {
        public static function dispatch(...$arguments)
        {
        }

        public static function dispatchIf($boolean, ...$arguments)
        {
        }

        public static function dispatchUnless($boolean, ...$arguments)
        {
        }
    }
}

namespace App {
    class User extends \Illuminate\Database\Eloquent\Model
    {
        public function group()
        {
        }
    }

    class Group extends \Illuminate\Database\Eloquent\Model
    {
    }
}

namespace Symfony\Component\Console\Command {
    class Command
    {
        protected function configure()
        {
        }

        public function addArgument(string $name, ?int $mode = null, string $description = '', mixed $default = null): static
        {
        }

        public function addOption(string $name, string|array|null $shortcut = null, ?int $mode = null, string $description = '', mixed $default = null): static
        {
        }
    }
}

namespace Symfony\Component\Console\Attribute {
    #[\Attribute(\Attribute::TARGET_CLASS)]
    class AsCommand
    {
        public function __construct(public string $name, public ?string $description = null, array $aliases = [], bool $hidden = false)
        {
        }
    }
}

namespace Symfony\Component\Console\Input {
    class InputArgument
    {
        public const REQUIRED = 1;
        public const OPTIONAL = 2;
        public const IS_ARRAY = 4;

        public function __construct(string $name, ?int $mode = null, string $description = '', mixed $default = null)
        {
        }
    }

    class InputOption
    {
        public const VALUE_NONE = 1;
        public const VALUE_REQUIRED = 2;
        public const VALUE_OPTIONAL = 4;
        public const VALUE_IS_ARRAY = 8;

        public function __construct(string $name, string|array|null $shortcut = null, ?int $mode = null, string $description = '', mixed $default = null)
        {
        }
    }
}

namespace Illuminate\Contracts\Console {
    interface Isolatable
    {
    }
}

namespace Illuminate\Console {
    class Command extends \Symfony\Component\Console\Command\Command
    {
        protected $signature;
        protected $name;
        protected $aliases;

        public function argument($key = null)
        {
        }

        public function option($key = null)
        {
        }

        protected function getArguments()
        {
            return [];
        }

        protected function getOptions()
        {
            return [];
        }
    }
}

namespace {
    function with($value, ?callable $callback = null)
    {
        return is_null($callback) ? $value : $callback($value);
    }

    function value($value, ...$args)
    {
        return $value instanceof Closure ? $value(...$args) : $value;
    }

    function env($key, $default = null)
    {
    }

    /**
     * @param  string  $path
     * @return string
     */
    function storage_path($path = '')
    {
    }

    /**
     * @param  string|null  $guard
     * @return ($guard is null ? \Illuminate\Contracts\Auth\Factory : \Illuminate\Contracts\Auth\Guard)
     */
    function auth($guard = null): \Illuminate\Contracts\Auth\Factory|\Illuminate\Contracts\Auth\Guard
    {
    }
}
