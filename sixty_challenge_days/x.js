
function fatorial(n) {
    if (n === 1 || n === 0) {
        return 1
    } else {
        return fatorial(n - 1) * n
    }
}

function fibonacci_bottom_up(nth) {
    const cache = new Map()
    return () => {
        if (nth <= 2) {
            return 1;
        }
        if (cache.has(nth)) {
            return cache.get(nth);
        }

        const result = fibonacci_bottom_up(nth - 1) + fibonacci_bottom_up(nth - 2);
        cache.set(nth, result);
        return result;
    }
}

/**
 * 
 * @param {Number[]} array 
 */
let count = ''
function qsort(array) {
    count += '*'
    console.log(count)
    if (array.length === 1 || array.length === 0) return array
    if (array.length === 2) {
        if (array[0] > array[1]) {
            const temp = array[0]
            array[0] = array[1]
            array[1] = temp
        }
        return array
    }
    const pivot = array.length - 1
    const less = []
    const greater = []
    const equals = []
    for (const i of array) {
        if (i < array[pivot]) {
            less.push(i)
        }
        if (i > array[pivot]) {
            greater.push(i)
        }
        if (i === array[pivot]) {
            equals.push(i)
        }
    }
    const r1 = qsort(less)
    count = count.slice(0, count.length - 1)
    const r2 = qsort(greater)
    count = count.slice(0, count.length - 1)
    console.log(count)
    const result = [...r1, ...equals, ...r2]
    return result

}

console.log(qsort([5, 3, 46, 2, 46, 25, 3, 5, 8, 9, 52, 2452, 52125, 1, 125, 1251, 125, 1251, 5, 151, 512, 51, 125125]))
console.log(fatorial(4))
console.log(fibonacci_bottom_up(7))