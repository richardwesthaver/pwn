;;; pr2.el --- Poor Richard's Pet Rat -*- lexical-binding: t; -*-

;; Copyright (C) 2022  ellis

;; Author: ellis <ellis@rwest.io>
;; Keywords: processes

;; This program is free software; you can redistribute it and/or modify
;; it under the terms of the GNU General Public License as published by
;; the Free Software Foundation, either version 3 of the License, or
;; (at your option) any later version.

;; This program is distributed in the hope that it will be useful,
;; but WITHOUT ANY WARRANTY; without even the implied warranty of
;; MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
;; GNU General Public License for more details.

;; You should have received a copy of the GNU General Public License
;; along with this program.  If not, see <https://www.gnu.org/licenses/>.

;;; Commentary:

;; https://rwest.io/pwn

;;; Code:
(require 'ewoc)

(defgroup pr2 nil
  "Poor Richard's Pet Rat")

(defcustom pr2-client-file-path "~/.cargo/bin/pr2-client"
  "path to the pr2-client binary"
  :group 'pr2)

(defcustom pr2-server-addr "127.0.0.1:9053"
  "socket address for pr2-server"
  :group 'pr2)

(defconst pr2-client-keywords
  '("help" "list" "get" "set" "quit"))

(defvar pr2-client-font-lock-keywords
  (list
   ;; highlight all opcodes.
   `(,(concat "\\_<" (regexp-opt pr2-client-keywords) "\\_>") . font-lock-keyword-face))
  "Additional expressions to highlight in `pr2-client-mode'.")

(defvar pr2-client-mode-map
  (let ((map (nconc (make-sparse-keymap) comint-mode-map)))
    (define-key map "\t" 'completion-at-point)
    map)
  "Basic mode map for `run-pr2-client'")

(defvar pr2-client-prompt-regexp "|| "
  "prompt for `run-pr2-client'")

;;;###autoload
(defun run-pr2-client (&optional addr)
  "run an inferior instance of pr2-client using remote address
ADDR. defaults to `pr2-server-addr'"
  (interactive)
  (let* ((prog pr2-client-file-path)
	 (buffer (comint-check-proc "pr2-client")))
    ;; pop to *pr2-client* buffer if the process is dead, the buffer
    ;; is missing or has the wrong mode.
    (pop-to-buffer-same-window
     (if (or buffer (not (derived-mode-p 'pr2-client-mode))
	     (comint-check-proc (current-buffer)))
	 (get-buffer-create (or buffer "*pr2-client*"))
       (current-buffer)))
    ;; create comint process if there is buffer is missing
    (unless buffer
      (apply 'make-comint-in-buffer "pr2-client" buffer prog nil (list "-s" (or addr pr2-server-addr)))
      (pr2-client-mode))))

(defun pr2-client--init ()
  "initialize pr2-client"
  (setq comint-process-echoes t
	comint-use-prompt-regexp t))

(define-derived-mode pr2-client-mode comint-mode "pr2-client"
  "Major mode for `run-pr2-client'
\\<pr2-client-mode-map>"
  nil "pr2-client"
  (setq comint-prompt-read-only t)
  (set (make-local-variable 'paragraph-separate) "\\'")
  (set (make-local-variable 'font-lock-defaults) '(pr2-client-font-lock-keywords t))
  (set (make-local-variable 'paragraph-start) pr2-client-prompt-regexp))

(add-hook 'pr2-client-mode-hook 'pr2-client--init)

(provide 'pr2)
;;; pr2.el ends here
