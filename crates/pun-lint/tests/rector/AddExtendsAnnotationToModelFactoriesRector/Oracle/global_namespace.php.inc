<?php

use App\Models\User;
use Illuminate\Database\Eloquent\Factories\Factory;

class UserFactory extends Factory
{
    public function definition(): array
    {
        return [];
    }

    protected $model = User::class;
}

class NoModelFactory extends Factory
{
    public function definition(): array
    {
        return [];
    }
}

class HookedFactory extends Factory
{
    public string $model = 'App\Models\Post' {
        get => $this->model;
    }
}

class ParentFactory extends Factory
{
    protected $model = parent::class;
}

class HeredocFactory extends Factory
{
    protected $model = <<<EOT
        App\Models\Comment
        EOT;
}

class NowdocFactory extends Factory
{
    protected $model = <<<'EOT'
        App\Models\Tag
        EOT;
}
-----
<?php

use App\Models\User;
use Illuminate\Database\Eloquent\Factories\Factory;

/**
 * @extends \Illuminate\Database\Eloquent\Factories\Factory<\App\Models\User>
 */
class UserFactory extends Factory
{
    public function definition(): array
    {
        return [];
    }

    protected $model = User::class;
}

class NoModelFactory extends Factory
{
    public function definition(): array
    {
        return [];
    }
}

/**
 * @extends \Illuminate\Database\Eloquent\Factories\Factory<\App\Models\Post>
 */
class HookedFactory extends Factory
{
    public string $model = 'App\Models\Post' {
        get => $this->model;
    }
}

/**
 * @extends \Illuminate\Database\Eloquent\Factories\Factory<\parent>
 */
class ParentFactory extends Factory
{
    protected $model = parent::class;
}

/**
 * @extends \Illuminate\Database\Eloquent\Factories\Factory<\App\Models\Comment>
 */
class HeredocFactory extends Factory
{
    protected $model = <<<EOT
        App\Models\Comment
        EOT;
}

/**
 * @extends \Illuminate\Database\Eloquent\Factories\Factory<\App\Models\Tag>
 */
class NowdocFactory extends Factory
{
    protected $model = <<<'EOT'
        App\Models\Tag
        EOT;
}
