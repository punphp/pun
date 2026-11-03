<?php

/*
 * This file is part of the Symfony package.
 *
 * (c) Fabien Potencier <fabien@symfony.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

namespace Symfony\Component\Console\Attribute;

use Symfony\Component\Console\Attribute\Reflection\ReflectionMember;
use Symfony\Component\Console\Exception\LogicException;
use Symfony\Component\Console\Input\InputInterface;
use Symfony\Component\Console\Question\ChoiceQuestion;
use Symfony\Component\Console\Style\SymfonyStyle;

#[\Attribute(\Attribute::TARGET_PARAMETER | \Attribute::TARGET_PROPERTY)]
class AskChoice implements InteractiveAttributeInterface
{
    public ?\Closure $validator;
    public array|\Closure $choices;
    private \Closure $closure;

    /**
     * @param string                                                     $question     The question to ask the user
     * @param array<string|int|float>|callable():array<string|int|float> $choices      The list of available choices (leave empty to use enum cases)
     * @param string|int|float|null                                      $default      The default answer to return if the user enters nothing
     * @param string                                                     $errorMessage The error message when the answer is invalid
     * @param string                                                     $prompt       The prompt displayed before the user input
     * @param callable|null                                              $validator    The validator for the answer
     * @param int|null                                                   $maxAttempts  The maximum number of attempts allowed to answer the question.
     *                                                                                 Null means an unlimited number of attempts
     */
    public function __construct(
        public string $question,
        array|callable $choices = [],
        public string|int|float|null $default = null,
        public string $errorMessage = 'Value "%s" is invalid',
        public string $prompt = ' > ',
        ?callable $validator = null,
        public ?int $maxAttempts = null,
    ) {}

    /**
     * @internal
     */
    public static function tryFrom(\ReflectionParameter|\ReflectionProperty $member, string $name): ?self
    {}

    /**
     * @internal
     */
    public function getFunction(object $instance): \ReflectionFunction
    {}
}
