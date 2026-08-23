/* OXIDE Community — Hacker Interface Logic */

(function() {
  'use strict';

  var urlIn     = document.getElementById('url-input');
  var conEl     = document.getElementById('console');
  var phEl      = document.getElementById('console-placeholder');
  var sd        = document.getElementById('status-dot');
  var st        = document.getElementById('status-text');
  var pw        = document.getElementById('progress-wrap');
  var pb        = document.getElementById('progress-bar');
  var btnStart  = document.getElementById('btn-start');
  var btnStop   = document.getElementById('btn-stop');
  var btnClear  = document.getElementById('btn-clear');

  var isScanning = false;

  function ao(text, cls) {
    phEl.style.display = 'none';
    var d = document.createElement('div');
    d.className = cls || 'out';
    d.textContent = text;
    conEl.appendChild(d);
    conEl.scrollTop = conEl.scrollHeight;
  }
  window.ao = ao;

  function setStatus(text, state) {
    st.textContent = text;
    sd.className = '';
    if (state) sd.classList.add(state);
  }
  window.setStatus = setStatus;

  function setRunning(r) {
    isScanning = r;
    btnStart.style.display = r ? 'none' : '';
    btnStop.style.display = r ? '' : 'none';
    if (r) {
      pw.classList.add('active');
      pb.classList.add('indeterminate');
      setStatus('SCANNING', 'scanning');
    } else {
      pb.classList.remove('indeterminate');
      setTimeout(function() { pw.classList.remove('active'); }, 600);
      setStatus('READY', 'ready');
    }
  }
  window.setRunning = setRunning;

  function applyPreset(mode) {
    var els = {
      threads:   document.getElementById('opt-threads'),
      level:     document.getElementById('opt-level'),
      payloads:  document.getElementById('opt-payloads'),
      rate:      document.getElementById('opt-rate'),
      duration:  document.getElementById('opt-duration'),
      depth:     document.getElementById('opt-depth'),
      maxurls:   document.getElementById('opt-maxurls'),
      jobs:      document.getElementById('opt-jobs'),
      redirects: document.getElementById('opt-redirects')
    };
    var chips = document.querySelectorAll('.mod-chip');

    function setChipsAll(active) {
      chips.forEach(function(e) {
        if (e.dataset.mod === 'all') {
          e.classList.toggle('active', active);
        } else {
          e.classList.toggle('active', !active);
        }
      });
    }
    function setChipsOnly(names) {
      chips.forEach(function(e) {
        e.classList.toggle('active', names.indexOf(e.dataset.mod) !== -1);
      });
    }

    var presets = {
      quick: function() {
        els.threads.value = 10; els.level.value = 30;
        els.payloads.value = 20; els.rate.value = 5;
        els.duration.value = 60; els.depth.value = 1;
        els.maxurls.value = 50; els.jobs.value = 1; els.redirects.value = 5;
        setChipsAll(true);
        ao('>> PRESET: QUICK SCAN — essential checks', 'info');
      },
      full: function() {
        els.threads.value = 50; els.level.value = 85;
        els.payloads.value = 100; els.rate.value = 0;
        els.duration.value = 0; els.depth.value = 5;
        els.maxurls.value = 1000; els.jobs.value = 4; els.redirects.value = 10;
        chips.forEach(function(e) { e.classList.add('active'); });
        ao('>> PRESET: FULL SCAN — all modules, deep', 'info');
      },
      stealth: function() {
        els.threads.value = 3; els.level.value = 10;
        els.payloads.value = 10; els.rate.value = 1;
        els.duration.value = 600; els.depth.value = 1;
        els.maxurls.value = 30; els.jobs.value = 1; els.redirects.value = 3;
        setChipsAll(true);
        ao('>> PRESET: STEALTH — low & slow evasion', 'info');
      },
      ml: function() {
        els.threads.value = 30; els.level.value = 60;
        els.payloads.value = 50; els.rate.value = 0;
        els.duration.value = 0; els.depth.value = 3;
        els.maxurls.value = 200; els.jobs.value = 2; els.redirects.value = 10;
        setChipsOnly(['all', 'zeroday', 'agent']);
        document.getElementById('toggle-zeroday').classList.add('on');
        document.getElementById('toggle-zeroday').classList.remove('off');
        ao('>> PRESET: ZERO-DAY — anomaly detection', 'info');
      },
      recon: function() {
        els.threads.value = 15; els.level.value = 20;
        els.payloads.value = 5; els.rate.value = 10;
        els.duration.value = 120; els.depth.value = 2;
        els.maxurls.value = 100; els.jobs.value = 3; els.redirects.value = 5;
        setChipsOnly(['recon', 'cors', 'tls']);
        ao('>> PRESET: RECON — surface info gathering', 'info');
      }
    };
    if (presets[mode]) presets[mode]();
  }

  function collectParams() {
    function v(id) { return document.getElementById(id).value; }
    function t(id) { return document.getElementById(id).classList.contains('on') ? 'true' : ''; }

    var mods = [];
    document.querySelectorAll('.mod-chip.active').forEach(function(e) {
      if (e.dataset.mod !== 'all') mods.push(e.dataset.mod);
    });
    if (mods.length === 0) mods.push('all');

    var proxyVal = v('opt-proxy').trim();
    var useBurp  = proxyVal.length > 0 ? 'true' : '';
    var useInsecure = (useBurp === 'true') ? 'true' : t('toggle-insecure');

    return {
      url:       urlIn.value.trim(),
      threads:   v('opt-threads'), level: v('opt-level'),
      payloads:  v('opt-payloads'), rate: v('opt-rate'),
      duration:  v('opt-duration'), format: v('opt-format'),
      depth:     v('opt-depth'), maxurls: v('opt-maxurls'),
      jobs:      v('opt-jobs'), redirects: v('opt-redirects'),
      modules:   mods.join(','),
      ua:        v('opt-ua'), cookie: v('opt-cookie'),
      burp:      useBurp, output: v('opt-output'),
      headers:   v('opt-headers'),
      follow:    t('toggle-follow'), insecure: useInsecure,
      verbose:   t('toggle-verbose'), silent: t('toggle-silent'),
      download:  t('toggle-download'),
      zeroday:   t('toggle-zeroday'), active: t('toggle-active'),
      train:     t('toggle-train'), multi: t('toggle-multi'),
      headless:  t('toggle-headless'), resume: t('toggle-resume'),
    };
  }

  function startScan() {
    var p = collectParams();
    if (!p.url) { ao('!! ERROR: No target URL specified', 'err'); return; }
    ao('>> LAUNCHING SCAN: ' + p.url, 'info');
    var qs = Object.keys(p).map(function(k) {
      return encodeURIComponent(k) + '=' + encodeURIComponent(p[k] || '');
    }).join('&');
    setRunning(true);
    fetch('oxide://scan/start?' + qs).catch(function(e) { ao('!! FETCH ERROR: ' + e, 'err'); });
  }

  function stopScan() {
    ao('!! SCAN HALTED BY USER', 'err');
    setRunning(false);
    fetch('oxide://scan/stop').catch(function(){});
  }

  function clearConsole() {
    conEl.innerHTML = '';
    phEl.style.display = 'flex';
  }

  btnStart.onclick = startScan;
  btnStop.onclick  = stopScan;
  btnClear.onclick = clearConsole;

  document.querySelectorAll('.preset').forEach(function(el) {
    el.onclick = function() {
      document.querySelectorAll('.preset').forEach(function(p) { p.style.borderColor = ''; });
      el.style.borderColor = 'var(--grv-cyan)';
      applyPreset(el.dataset.mode);
    };
  });

  document.querySelectorAll('.mod-chip').forEach(function(el) {
    el.onclick = function() {
      if (this.dataset.mod === 'all') {
        document.querySelectorAll('.mod-chip').forEach(function(e) { e.classList.remove('active'); });
        this.classList.add('active');
      } else {
        this.classList.toggle('active');
        document.querySelector('.mod-chip[data-mod="all"]').classList.remove('active');
      }
    };
  });

  document.querySelectorAll('.toggle-switch').forEach(function(el) {
    el.onclick = function() {
      this.classList.toggle('on');
      this.classList.toggle('off');
    };
  });

  document.querySelectorAll('.custom-select').forEach(function(wrap) {
    var trigger = wrap.querySelector('.custom-select-trigger');
    var panel = wrap.querySelector('.custom-select-options');
    var options = wrap.querySelectorAll('.custom-select-option');
    var valueEl = wrap.querySelector('.custom-select-value');
    var hidden = wrap.nextElementSibling;
    function pos() {
      var r = trigger.getBoundingClientRect();
      panel.style.top = (r.bottom) + 'px';
      panel.style.left = r.left + 'px';
      panel.style.width = r.width + 'px';
    }
    trigger.onclick = function(e) {
      e.stopPropagation();
      document.querySelectorAll('.custom-select.open').forEach(function(s) { if (s !== wrap) s.classList.remove('open'); });
      if (!wrap.classList.contains('open')) pos();
      wrap.classList.toggle('open');
    };
    options.forEach(function(opt) {
      opt.onclick = function() {
        options.forEach(function(o) { o.classList.remove('selected'); });
        this.classList.add('selected');
        valueEl.textContent = this.textContent;
        if (hidden) hidden.value = this.dataset.value;
        wrap.classList.remove('open');
      };
    });
  });
  document.onclick = function() {
    document.querySelectorAll('.custom-select.open').forEach(function(s) { s.classList.remove('open'); });
  };

  document.querySelectorAll('.opt-group-header').forEach(function(el) {
    el.onclick = function() {
      this.nextElementSibling.classList.toggle('open');
    };
  });

  document.getElementById('btn-targets').onclick = function() {
    fetch('oxide://pickfile').then(function(r) { return r.text(); }).then(function(f) {
      if (f) urlIn.value = f;
    }).catch(function(){});
  };

  document.getElementById('info-btn').onclick = function() {
    document.getElementById('about-overlay').style.display = '';
  };
  document.getElementById('about-close').onclick = function() {
    document.getElementById('about-overlay').style.display = 'none';
  };
  document.getElementById('about-overlay').onclick = function(e) {
    if (e.target === this) this.style.display = 'none';
  };

  // custom title bar
  document.getElementById('win-min').onclick = function() { fetch('oxide://window/minimize').catch(function(){}); };
  document.getElementById('win-max').onclick = function() { fetch('oxide://window/maximize').catch(function(){}); };
  document.getElementById('win-close').onclick = function() { fetch('oxide://window/close').catch(function(){}); };
  var dh = {x:0,y:0,active:false};
  document.getElementById('header').onmousedown = function(e) {
    if (e.target.closest('button,.win-btn,#info-btn,#win-controls')) return;
    dh.x=e.clientX; dh.y=e.clientY; dh.active=true;
  };
  document.onmousemove = function(e) {
    if (!dh.active) return;
    if (Math.abs(e.clientX-dh.x)>3||Math.abs(e.clientY-dh.y)>3) {
      dh.active=false;
      fetch('oxide://window/drag').catch(function(){});
    }
  };
  document.onmouseup = function() { dh.active=false; };

  document.onkeydown = function(e) {
    if (e.ctrlKey && e.key === 'Enter') { e.preventDefault(); startScan(); }
    if (e.key === 'Escape') {
      var ao = document.getElementById('about-overlay');
      if (ao.style.display !== 'none' && ao.style.display !== '') {
        e.preventDefault(); ao.style.display = 'none'; return;
      }
      if (isScanning) { e.preventDefault(); stopScan(); }
    }
  };

  setTimeout(function() {
    var s = document.getElementById('splash');
    if (s) s.classList.add('h');
  }, 1800);
})();
