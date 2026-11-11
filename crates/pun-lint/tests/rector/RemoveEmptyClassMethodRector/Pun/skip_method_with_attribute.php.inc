<?php

namespace Rector\Tests\DeadCode\Rector\ClassMethod\RemoveEmptyClassMethodRector\Pun;

use Livewire\Attributes\On;

// Rector 2.8.4 removes refresh(), which Livewire calls for the event. Pun keeps a method that carries an attribute.
final class SkipMethodWithAttribute
{
    #[On('question.created')]
    public function refresh(): void {}

    public function unused(): void {}
}

?>
-----
<?php

namespace Rector\Tests\DeadCode\Rector\ClassMethod\RemoveEmptyClassMethodRector\Pun;

use Livewire\Attributes\On;

// Rector 2.8.4 removes refresh(), which Livewire calls for the event. Pun keeps a method that carries an attribute.
final class SkipMethodWithAttribute
{
    #[On('question.created')]
    public function refresh(): void {}
}

?>
