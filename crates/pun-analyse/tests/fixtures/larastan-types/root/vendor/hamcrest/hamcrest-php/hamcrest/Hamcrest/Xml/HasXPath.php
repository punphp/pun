<?php
namespace Hamcrest\Xml;

/*
 Copyright (c) 2009 hamcrest.org
 */
use Hamcrest\Core\IsEqual;
use Hamcrest\Description;
use Hamcrest\DiagnosingMatcher;
use Hamcrest\Matcher;

/**
 * Matches if XPath applied to XML/HTML/XHTML document either
 * evaluates to result matching the matcher or returns at least
 * one node, matching the matcher if present.
 */
class HasXPath extends DiagnosingMatcher
{

    /**
     * XPath to apply to the DOM.
     *
     * @var string
     */
    private string $_xpath;

    /**
     * Optional matcher to apply to the XPath expression result
     * or the content of the returned nodes.
     *
     * @var ?Matcher
     */
    private ?Matcher $_matcher;

    /**
     * @param string $xpath
     */
    public function __construct(string $xpath, ?Matcher $matcher = null)
    {}

    /**
     * Matches if the XPath matches against the DOM node and the matcher.
     *
     * @param string|\DOMNode $actual
     * @param Description $mismatchDescription
     * @return bool
     */
    protected function matchesWithDiagnosticDescription($actual, Description $mismatchDescription): bool
    {}

    /**
     * Creates and returns a <code>DOMDocument</code> from the given
     * XML or HTML string.
     *
     * @param string $text
     * @return \DOMDocument built from <code>$text</code>
     * @throws \InvalidArgumentException if the document is not valid
     */
    protected function createDocument($text)
    {}

    /**
     * Applies the configured XPath to the DOM node and returns either
     * the result if it's an expression or the node list if it's a query.
     *
     * @param \DOMNode $node context from which to issue query
     * @return mixed result of expression or DOMNodeList from query
     */
    protected function evaluate(\DOMNode $node)
    {}

    /**
     * Matches if the list of nodes is not empty and the content of at least
     * one node matches the configured matcher, if supplied.
     *
     * @param \DOMNodeList $nodes selected by the XPath query
     * @param Description $mismatchDescription
     * @return bool
     */
    protected function matchesContent(\DOMNodeList $nodes, Description $mismatchDescription): bool
    {}

    /**
     * Matches if the result of the XPath expression matches the configured
     * matcher or evaluates to <code>true</code> if there is none.
     *
     * @param mixed $result result of the XPath expression
     * @param Description $mismatchDescription
     * @return bool
     */
    protected function matchesExpression($result, Description $mismatchDescription): bool
    {}

    public function describeTo(Description $description): void
    {}

    /**
     * Wraps <code>$matcher</code> with {@link Hamcrest\Core\IsEqual)
     * if it's not a matcher and the XPath in <code>count()</code>
     * if it's an integer.
     *
     * @factory
     * @param string $xpath
     * @param null|Matcher|int|mixed $matcher
     */
    public static function hasXPath(string $xpath, $matcher = null): self
    {}
}
