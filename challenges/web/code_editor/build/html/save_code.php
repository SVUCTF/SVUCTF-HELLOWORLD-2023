<?php
header('Content-Type: application/json');

$data = json_decode(file_get_contents('php://input'), true);

if (isset($data['code']) && isset($data['filename'])) {
    $code = $data['code'];
    $filename = $data['filename'];

    if (file_put_contents('saved/' . $filename, $code)) {
        echo json_encode(['success' => true]);
    } else {
        echo json_encode(['success' => false]);
    }
} else {
    echo json_encode(['success' => false]);
}
?>

