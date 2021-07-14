module.exports = class Queue {
  constructor(limit){
    this.queue = [];
    this.limit = limit;
  }

  size(){
    return this.queue.length;
  }

  append(item){
    this.queue.push(item);
    if (this.limit && this.size() > this.limit) {
      this.queue.shift()
    }
  }

  peek(){
    return this.queue;
  }

  clear(){
    this.queue = [];
  }
}