<?php declare(strict_types=1);

namespace PipeRuleObjects;

final class Reference
{
    public function __invoke(string &$invoked): string { return $invoked; }
    public static function staticReference(string &$static): string { return $static; }
    public function instanceReference(string &$instance): string { return $instance; }
    private static function hidden(string &$private): string { return $private; }

    public function examples(): void
    {
        'text' |> $this;
        'text' |> (
            $this
        );
        'text' |> self::staticReference(...);
        'text' |> $this->instanceReference(...);
        'text' |> [self::class, 'staticReference'];
        'text' |> [$this, 'instanceReference'];
        'text' |> self::hidden(...);
    }
}

final class Plain {}

function examples(Reference $reference, Plain $plain, MissingClass $missing, object $object): void
{
    'text' |> $reference;
    'text' |> $plain;
    'text' |> $missing;
    'text' |> $object;
    'text' |> ['PipeRuleObjects\Reference', 'staticReference'];
    'text' |> 'PipeRuleObjects\Reference::staticReference';
    'text' |> ['PipeRuleObjects\Reference', 'hidden'];
    'text' |> ['PipeRuleObjects\Reference', 'missing'];
}
