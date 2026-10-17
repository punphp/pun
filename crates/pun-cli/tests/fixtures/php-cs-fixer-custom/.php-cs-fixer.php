<?php

require_once __DIR__.'/UpperTodoFixer.php';

return (new PhpCsFixer\Config())
    ->registerCustomFixers([new UpperTodoFixer()])
    ->setRules(['@PSR12' => true, 'single_quote' => true, 'Acme/upper_todo' => true])
    ->setFinder(PhpCsFixer\Finder::create()->in(__DIR__.'/app'));
