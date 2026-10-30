<?php
namespace CloneNeverControls;

// The capture's dynamic return extension supplies each bottom identity without
// turning these mixed-returning declarations into intrinsically terminating calls.
function implicitBottom(): mixed { return 1; }
function explicitBottom(): mixed { return 1; }
function nonacceptingBottom(): mixed { return 1; }
function terminate(): never { exit(); }

function implicitCase(): void {
    clone implicitBottom();
    'after-implicit';
}
function explicitCase(): void {
    clone explicitBottom();
    'after-explicit';
}
function nonacceptingCase(): void {
    clone nonacceptingBottom();
    'after-nonaccepting';
}
function computedCase(): void {
    clone 1;
    'after-computed';
}
function forwardedNamed(): void {
    clone \call_user_func(terminate(...));
    'after-forwarded-named';
}
function forwardedNonaccepting(): void {
    clone \call_user_func(function () { exit(); });
    'after-forwarded-nonaccepting';
}
function nestedForwarding(): void {
    consume(clone \call_user_func(terminate(...)));
    'after-nested-forwarding';
}
function consume(mixed $value): void {}
