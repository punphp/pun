<?php
namespace Hamcrest\Core;

/*
 Copyright (c) 2009 hamcrest.org
 */
use Hamcrest\BaseMatcher;
use Hamcrest\Description;
use Hamcrest\Matcher;

/**
 * Provides a custom description to another matcher.
 */
class DescribedAs extends BaseMatcher
{

    private string $_descriptionTemplate;
    private Matcher $_matcher;
    /**
     * @var array<mixed>
     */
    private array $_values;

    private const ARG_PATTERN = '/%([0-9]+)/';

    /**
     * @param array<mixed> $values
     */
    public function __construct(string $descriptionTemplate, Matcher $matcher, array $values)
    {}

    public function matches($item): bool
    {}

    public function describeTo(Description $description): void
    {}

    /**
     * Wraps an existing matcher and overrides the description when it fails.
     *
     * @factory ...
     */
    public static function describedAs(/* $description, Hamcrest\Matcher $matcher, $values... */): self
    { func_get_args(); }
}
