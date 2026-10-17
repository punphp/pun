<?php

// The Laravel and Carbon classes the fixture names, for Pun's test (the oracle run has the real vendor).

namespace Illuminate\Database\Eloquent {
    abstract class Model
    {
        public function __get($key)
        {
            return null;
        }
    }

    trait SoftDeletes
    {
    }
}

namespace Illuminate\Database\Eloquent\Casts {
    /** @template TGet @template TSet */
    final class Attribute
    {
        public static function make(?callable $get = null, ?callable $set = null): static
        {
            return new static();
        }
    }
}

namespace Illuminate\Contracts\Database\Eloquent {
    interface CastsAttributes
    {
    }

    interface CastsInboundAttributes
    {
    }
}

namespace Illuminate\Database\Eloquent\Factories {
    trait HasFactory
    {
    }
}

namespace Illuminate\Database\Eloquent\Relations {
    class BelongsTo
    {
    }
}

namespace Illuminate\Database\Migrations {
    abstract class Migration
    {
    }
}

namespace Illuminate\Database\Schema {
    class Blueprint
    {
    }
}

namespace Illuminate\Support {
    abstract class ServiceProvider
    {
        public $app;
    }
}

namespace Illuminate\Support\Facades {
    class Facade
    {
    }

    class Schema extends Facade
    {
    }

    class DB extends Facade
    {
    }

    class Date extends Facade
    {
    }
}

namespace Carbon {
    class Carbon
    {
    }

    class CarbonImmutable
    {
    }
}
