import http from "k6/http";
import { check, sleep } from "k6";
import { randomString } from "https://jslib.k6.io/k6-utils/1.2.0/index.js";

export const options = {
  summaryTrendStats: ["avg", "min", "med", "max", "p(90)", "p(95)", "p(99)"],
  scenarios: {
    registration_load: {
      executor: "constant-arrival-rate",
      rate: 25000,
      timeUnit: "1s",
      duration: "1m",
      preAllocatedVUs: 1000,
      maxVUs: 5000,
    },
  },
  thresholds: {
    http_req_failed: ["rate<0.01"],
    http_req_duration: ["p(95)<2000"],
  },
};

export default function () {
  const email = `${randomString(10)}@example.com`;
  const name = randomString(8);
  const password = "Password@123";

  const payload = JSON.stringify({
    email: email,
    name: name,
    password: password,
  });

  const params = {
    headers: {
      "Content-Type": "application/json",
    },
  };

  const res = http.post("http://localhost:4000/auth/register", payload, params);

  check(res, {
    "status is 200": (r) => r.status === 200,
  });
}
