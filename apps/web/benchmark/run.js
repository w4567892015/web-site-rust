import http from 'k6/http';
import { check, sleep } from 'k6';

export const options = {
  scenarios: {
    shared_iter_scenario: {
      executor: 'shared-iterations',
      vus: 10,
      iterations: 100,
      startTime: '0s',
    },
    per_vu_scenario: {
      executor: 'per-vu-iterations',
      vus: 10,
      iterations: 10,
      startTime: '10s',
    },
  },
};

export default function () {
  const res = http.get('http://127.0.0.1');
  check(res, { 'status was 200': (r) => r.status == 200 });
  sleep(1);
}
