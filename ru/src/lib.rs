const y = new Proxy([], {
  get(_, key) {
    console.log(typeof(y))
    return _[key];
  }
});
        
