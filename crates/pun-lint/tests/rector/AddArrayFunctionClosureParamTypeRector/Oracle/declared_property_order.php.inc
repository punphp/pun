<?php

namespace Probe;

final class AcceptedIf
{
    /**
     * @var array<int, string|int|float|bool|null>
     */
    protected array $values;

    /**
     * @var array<string, \Closure|string>
     */
    protected $wildcards = [];

    public function __construct(string|int|float|bool|null|array $values)
    {
        $this->values = is_array($values) ? $values : array_slice(func_get_args(), 1);
    }

    public function toArray(): array
    {
        return array_map(
            static fn ($value) => is_string($value) ? $value : json_encode($value),
            $this->values,
        );
    }

    public function has($eventName)
    {
        return array_filter($this->wildcards, fn ($listeners, $key) => $key === $eventName, ARRAY_FILTER_USE_BOTH);
    }

    public function filter(string $messageId, ?string $receiptHandle, ?int $delay = null)
    {
        return array_filter([
            'messageId' => $messageId,
            'receiptHandle' => $receiptHandle,
            'delay' => $delay,
        ], fn ($value) => $value !== null);
    }
}
-----
<?php

namespace Probe;

final class AcceptedIf
{
    /**
     * @var array<int, string|int|float|bool|null>
     */
    protected array $values;

    /**
     * @var array<string, \Closure|string>
     */
    protected $wildcards = [];

    public function __construct(string|int|float|bool|null|array $values)
    {
        $this->values = is_array($values) ? $values : array_slice(func_get_args(), 1);
    }

    public function toArray(): array
    {
        return array_map(
            static fn (string|int|float|bool|null $value) => is_string($value) ? $value : json_encode($value),
            $this->values,
        );
    }

    public function has($eventName)
    {
        return array_filter($this->wildcards, fn ($listeners, $key) => $key === $eventName, ARRAY_FILTER_USE_BOTH);
    }

    public function filter(string $messageId, ?string $receiptHandle, ?int $delay = null)
    {
        return array_filter([
            'messageId' => $messageId,
            'receiptHandle' => $receiptHandle,
            'delay' => $delay,
        ], fn (string|int|null $value) => $value !== null);
    }
}
