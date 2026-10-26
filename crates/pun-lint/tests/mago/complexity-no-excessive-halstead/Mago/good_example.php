<?php

function processOrderData($orders) {
    $filtered = filterValidOrders($orders);
    $totals = calculateTotals($filtered);
    return applyDiscounts($totals);
}

function filterValidOrders($orders) {
    return array_filter($orders, fn($order) => $order['status'] === 'valid');
}

function calculateTotals($orders) {
    return array_map(fn($order) => $order['price'] * $order['quantity'], $orders);
}

function applyDiscounts($totals) {
    return array_map(fn($total) => $total > 100 ? $total * 0.9 : $total, $totals);
}
