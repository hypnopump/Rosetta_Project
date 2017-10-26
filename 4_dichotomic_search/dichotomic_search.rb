def assert_equal(actual, expect)
  raise 'Assertion failed!' unless actual == expect
end

def dicho_search(arr, val)
  mid = arr.size / 2
  if arr.size > 1
    return dicho_search(arr[0...mid], val) if arr[mid] > val
    return dicho_search(arr[mid..-1], val)
  end
  return val if arr[0] == val
  nil
end

assert_equal(dicho_search(%w[a b d e g h j l m q x], 'z'), nil)
assert_equal(dicho_search(%w[a b d e g h j l m q x], 'e'), 'e')
assert_equal(dicho_search([1, 2, 3, 5, 6, 10, 15], 20), nil)
assert_equal(dicho_search([1, 2, 3, 5, 6, 10, 15], 4), nil)
assert_equal(dicho_search([1, 2, 3, 5, 6, 10, 15], 3), 3)
