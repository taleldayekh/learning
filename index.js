const { Observable } = require('rxjs');

const observable = new Observable((subscriber) => {
    subscriber.next(10);
});

const observer = {
    next: (value) => { console.log(value) },
    error: (error) => { console.log(error) },
    complete: () => {
        console.log('Observer completed')
    },
}

observable.subscribe(observer);