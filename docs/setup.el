;;; setup.el --- Babel setup for TTS backend org file -*- lexical-binding: t; -*-

;; Copyright (c) 2025 Michael A. Wright
;; License: MIT

;;; Commentary:

;; This file configures Emacs org-babel for executing the bash blocks
;; in backend-setup.org. Copy both files to your backend server and
;; evaluate this file before working with the org document.
;;
;; Usage:
;;   1. scp docs/setup.el docs/backend-setup.org user@backend:~/tts/
;;   2. ssh user@backend
;;   3. cd ~/tts && emacs -l setup.el backend-setup.org
;;
;; Or from within Emacs:
;;   M-x load-file RET ~/tts/setup.el RET
;;   C-x C-f ~/tts/backend-setup.org RET

;;; Code:

;; Enable babel languages
(org-babel-do-load-languages
 'org-babel-load-languages
 '((shell . t)
   (emacs-lisp . t)))

;; Don't ask for confirmation before executing code blocks
;; WARNING: Only use this on trusted org files!
(setq org-confirm-babel-evaluate nil)

;; Use bash as the default shell
(setq org-babel-default-header-args:sh
      '((:results . "output")
        (:exports . "both")
        (:shell . "/bin/bash")))

;; Alias 'bash' to 'sh' for org-babel
(defalias 'org-babel-execute:bash 'org-babel-execute:sh)

;; Set default directory for code blocks
(setq org-babel-default-header-args
      (cons '(:dir . "/tmp/tts-backend")
            org-babel-default-header-args))

;; Enable syntax highlighting in code blocks
(setq org-src-fontify-natively t)
(setq org-src-tab-acts-natively t)

;; Preserve indentation in code blocks
(setq org-src-preserve-indentation t)
(setq org-edit-src-content-indentation 0)

;; Display images inline after execution
(setq org-startup-with-inline-images t)
(add-hook 'org-babel-after-execute-hook 'org-display-inline-images)

;; Auto-save org file after executing blocks
(add-hook 'org-babel-after-execute-hook 'save-buffer)

;; Helper function to execute all blocks in a section
(defun tts-execute-section ()
  "Execute all source blocks in the current org section."
  (interactive)
  (save-excursion
    (org-back-to-heading t)
    (let ((end (save-excursion (org-end-of-subtree t) (point))))
      (while (re-search-forward org-babel-src-block-regexp end t)
        (org-babel-execute-src-block)))))

(global-set-key (kbd "C-c C-v s") 'tts-execute-section)

;; Helper function to check prerequisites
(defun tts-check-prerequisites ()
  "Check that required tools are installed."
  (interactive)
  (let ((missing nil))
    (dolist (cmd '("docker" "nvidia-smi" "curl" "git"))
      (unless (executable-find cmd)
        (push cmd missing)))
    (if missing
        (message "Missing required commands: %s" (string-join missing ", "))
      (message "All prerequisites found!"))))

;; Helper to show docker status
(defun tts-docker-status ()
  "Show status of TTS docker containers."
  (interactive)
  (let ((buf (get-buffer-create "*TTS Docker Status*")))
    (with-current-buffer buf
      (erase-buffer)
      (insert "=== Docker Containers ===\n\n")
      (call-process "docker" nil buf nil "compose" "-f" "/tmp/tts-backend/docker-compose.yml" "ps")
      (insert "\n\n=== GPU Status ===\n\n")
      (call-process "nvidia-smi" nil buf nil))
    (display-buffer buf)))

(global-set-key (kbd "C-c t s") 'tts-docker-status)
(global-set-key (kbd "C-c t c") 'tts-check-prerequisites)

;; Message on load
(message "TTS backend setup loaded. Use C-c C-c to execute blocks, C-c t s for status.")

(provide 'setup)
;;; setup.el ends here
