<?php

final class Store
{
    /** @var array<string, callable> */
    private $extensions = [];

    /** @var array<string, array<string>> */
    private $messages = [];

    /** @var array<int, string> */
    private array $names = [];

    private $untyped = [];

    public function extend($name, callable $resolver)
    {
        $this->extensions[$name] = $resolver;
    }

    public function name($id, string $name)
    {
        $this->names[$id] = $name;
    }

    public function untyped($key)
    {
        $this->untyped[$key] = 1;
    }

    public function isUnique($key, $message)
    {
        $messages = (array) $this->messages;

        return ! isset($messages[$key]) || ! in_array($message, $messages[$key]);
    }

    public function other($key)
    {
        $messages = ['a' => 1];
        $others = ['b' => 2];

        return isset($others['b']) && $messages[$key];
    }

    public function issetOnly($key)
    {
        $messages = ['a' => 1];

        return isset($messages[$key]);
    }
}
-----
<?php

final class Store
{
    /** @var array<string, callable> */
    private $extensions = [];

    /** @var array<string, array<string>> */
    private $messages = [];

    /** @var array<int, string> */
    private array $names = [];

    private $untyped = [];

    public function extend(string $name, callable $resolver)
    {
        $this->extensions[$name] = $resolver;
    }

    public function name(int $id, string $name)
    {
        $this->names[$id] = $name;
    }

    public function untyped($key)
    {
        $this->untyped[$key] = 1;
    }

    public function isUnique($key, $message)
    {
        $messages = (array) $this->messages;

        return ! isset($messages[$key]) || ! in_array($message, $messages[$key]);
    }

    public function other(string $key)
    {
        $messages = ['a' => 1];
        $others = ['b' => 2];

        return isset($others['b']) && $messages[$key];
    }

    public function issetOnly(string $key)
    {
        $messages = ['a' => 1];

        return isset($messages[$key]);
    }
}
